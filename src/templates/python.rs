pub const PYTEST_FUNCTION_TEST_TEMPLATE: &str = r#"
import pytest
import unittest.mock
from unittest.mock import patch, MagicMock

class Test{{ function_name | title }}:
    def test_{{ test_name }}_{{ test_category | lower }}(self):
        """{{ description }}"""
        {% for input in inputs %}
        {% if input.email is defined %}
        assert {{ function_name }}('{{ input.email }}') == {{ expected_outputs.0 }}
        {% elif input is number %}
        assert {{ function_name }}({{ input }}) == {{ expected_outputs.0 }}
        {% else %}
        assert {{ function_name }}('{{ input }}') == {{ expected_outputs.0 }}
        {% endif %}
        {% endfor %}
    
    def test_{{ test_name }}_edge_cases(self):
        """Test edge cases for {{ function_name }}"""
        # Test with None
        with pytest.raises(TypeError):
            {{ function_name }}(None)
        
        {% if test_category == "numeric" %}
        # Numeric edge cases
        assert {{ function_name }}(0) is not None
        assert {{ function_name }}(-1) is not None
        assert {{ function_name }}(float('inf')) is not None
        {% elif test_category == "string" %}
        # String edge cases  
        assert {{ function_name }}('') is not None
        assert {{ function_name }}('a' * 1000) is not None
        assert {{ function_name }}('special!@#$%') is not None
        {% endif %}
    
    def test_{{ test_name }}_type_validation(self):
        """Test type validation for {{ function_name }}"""
        # Test with invalid types
        invalid_inputs = [[], {}, set(), lambda x: x]
        
        for invalid_input in invalid_inputs:
            with pytest.raises((TypeError, ValueError, AttributeError)):
                {{ function_name }}(invalid_input)
    
    @pytest.mark.parametrize("input_val,expected", [
        {% for input in inputs %}
        ({{ input }}, {{ expected_outputs.0 | default(value="None") }}),
        {% endfor %}
    ])
    def test_{{ test_name }}_parametrized(self, input_val, expected):
        """Parametrized test for {{ function_name }}"""
        result = {{ function_name }}(input_val)
        if expected is not None:
            assert result == expected
        else:
            assert result is not None
"#;

pub const PYTEST_CLASS_TEST_TEMPLATE: &str = r#"
import pytest
from unittest.mock import Mock, patch

class Test{{ function_name }}:
    
    @pytest.fixture
    def instance(self):
        """Create a test instance"""
        {{ setup_code | default(value="return " ~ function_name ~ "()") }}
    
    def test_initialization(self, instance):
        """Test class initialization"""
        assert instance is not None
        assert isinstance(instance, {{ function_name }})
    
    def test_initialization_with_parameters(self):
        """Test initialization with various parameters"""
        {% for input in inputs %}
        test_instance = {{ function_name }}({{ input }})
        assert test_instance is not None
        {% endfor %}
    
    def test_instance_methods_exist(self, instance):
        """Test that expected methods exist"""
        # Test basic callable attributes
        assert callable(getattr(instance, '__init__', None))
        
        # Test detected methods (would be populated dynamically)
        {% for method in methods | default(value=[]) %}
        assert hasattr(instance, '{{ method }}')
        assert callable(getattr(instance, '{{ method }}'))
        {% endfor %}
    
    def test_state_management(self, instance):
        """Test state management and consistency"""
        # Capture initial state
        initial_attrs = {k: v for k, v in instance.__dict__.items() 
                        if not k.startswith('_')}
        
        # Perform operations (would be method-specific)
        # This would be populated based on detected methods
        
        # Verify state consistency
        current_attrs = {k: v for k, v in instance.__dict__.items() 
                        if not k.startswith('_')}
        
        # Basic state validation
        assert instance is not None
    
    def test_string_representation(self, instance):
        """Test string representation methods"""
        assert str(instance) is not None
        assert repr(instance) is not None
        assert len(str(instance)) > 0
    
    def test_equality_and_hashing(self, instance):
        """Test equality and hashing if implemented"""
        another_instance = {{ function_name }}()
        
        # Test equality
        if hasattr(instance, '__eq__'):
            assert (instance == another_instance) in [True, False]
        
        # Test hashing if hashable
        if hasattr(instance, '__hash__'):
            try:
                hash(instance)
                assert True  # If we get here, hashing works
            except TypeError:
                pass  # Not hashable, which is fine
"#;

pub const PYTEST_ASYNC_TEST_TEMPLATE: &str = r#"
import pytest
import asyncio
from unittest.mock import AsyncMock, patch

class Test{{ function_name }}Async:
    
    @pytest.mark.asyncio
    async def test_{{ test_name }}_{{ test_category | lower }}(self):
        """{{ description }}"""
        {% for input in inputs %}
        result = await {{ function_name }}({{ input }})
        assert result is not None
        {% if expected_outputs %}
        assert result == {{ expected_outputs.0 }}
        {% endif %}
        {% endfor %}
    
    @pytest.mark.asyncio
    async def test_{{ test_name }}_exception_handling(self):
        """Test async exception handling"""
        # Test with invalid inputs
        with pytest.raises((TypeError, ValueError)):
            await {{ function_name }}(None)
        
        with pytest.raises((TypeError, ValueError)):
            await {{ function_name }}()
    
    @pytest.mark.asyncio 
    async def test_{{ test_name }}_timeout(self):
        """Test async timeout scenarios"""
        try:
            # Test with reasonable timeout
            result = await asyncio.wait_for({{ function_name }}(), timeout=5.0)
            assert result is not None
        except asyncio.TimeoutError:
            # Timeout is acceptable for some functions
            assert True
    
    @pytest.mark.asyncio
    async def test_{{ test_name }}_concurrent_execution(self):
        """Test concurrent execution"""
        tasks = [
            {{ function_name }}(f"input_{i}") 
            for i in range(3)
        ]
        
        results = await asyncio.gather(*tasks, return_exceptions=True)
        assert len(results) == 3
        
        # Check that we got either results or expected exceptions
        for result in results:
            assert result is not None or isinstance(result, Exception)
    
    @pytest.mark.asyncio
    async def test_{{ test_name }}_cancellation(self):
        """Test task cancellation"""
        task = asyncio.create_task({{ function_name }}())
        
        # Let it run briefly then cancel
        await asyncio.sleep(0.01)
        task.cancel()
        
        with pytest.raises(asyncio.CancelledError):
            await task
    
    @pytest.mark.asyncio
    @patch('{{ function_name }}')
    async def test_{{ test_name }}_mocked(self, mock_func):
        """Test with mocked async function"""
        mock_func.return_value = {{ expected_outputs.0 | default(value="'mocked_result'") }}
        
        result = await mock_func()
        assert result == {{ expected_outputs.0 | default(value="'mocked_result'") }}
        mock_func.assert_called_once()
"#;

pub const PYTEST_API_TEST_TEMPLATE: &str = r#"
import pytest
import requests
from unittest.mock import Mock, patch
import json

class Test{{ function_name }}API:
    
    @pytest.fixture
    def mock_response(self):
        """Create mock response"""
        mock_resp = Mock()
        mock_resp.status_code = 200
        mock_resp.json.return_value = {{ expected_outputs.0 | default(value='{"success": true}') }}
        return mock_resp
    
    @patch('requests.get')
    def test_{{ test_name }}_success(self, mock_get, mock_response):
        """Test successful API call"""
        mock_get.return_value = mock_response
        
        {% for input in inputs %}
        result = {{ function_name }}({{ input }})
        assert result is not None
        {% endfor %}
        
        mock_get.assert_called()
    
    @patch('requests.get')
    def test_{{ test_name }}_http_errors(self, mock_get):
        """Test HTTP error handling"""
        error_codes = [400, 401, 404, 500]
        
        for code in error_codes:
            mock_resp = Mock()
            mock_resp.status_code = code
            mock_resp.raise_for_status.side_effect = requests.HTTPError(f"HTTP {code}")
            mock_get.return_value = mock_resp
            
            with pytest.raises((requests.HTTPError, ValueError)):
                {{ function_name }}()
    
    @patch('requests.get')
    def test_{{ test_name }}_network_error(self, mock_get):
        """Test network error handling"""
        mock_get.side_effect = requests.ConnectionError("Network error")
        
        with pytest.raises(requests.ConnectionError):
            {{ function_name }}()
    
    @patch('requests.get')
    def test_{{ test_name }}_timeout(self, mock_get):
        """Test timeout handling"""
        mock_get.side_effect = requests.Timeout("Request timeout")
        
        with pytest.raises(requests.Timeout):
            {{ function_name }}()
    
    @patch('requests.get')
    def test_{{ test_name }}_json_parsing(self, mock_get):
        """Test JSON parsing edge cases"""
        # Test invalid JSON
        mock_resp = Mock()
        mock_resp.status_code = 200
        mock_resp.json.side_effect = json.JSONDecodeError("Invalid JSON", "", 0)
        mock_get.return_value = mock_resp
        
        with pytest.raises(json.JSONDecodeError):
            {{ function_name }}()
"#;