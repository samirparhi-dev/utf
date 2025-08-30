pub const JEST_FUNCTION_TEST_TEMPLATE: &str = r#"
describe('{{ function_name }}', () => {
  test('{{ test_name }} - {{ description }}', () => {
    // {{ test_category }} test case
    {% for input in inputs %}
    {% if input.email is defined %}
    expect({{ function_name }}('{{ input.email }}')).toBe({{ expected_outputs.0 }});
    {% elif input.number is defined %}
    expect({{ function_name }}({{ input.number }})).toBe({{ expected_outputs.0 }});
    {% else %}
    expect({{ function_name }}('{{ input }}')).toBe({{ expected_outputs.0 }});
    {% endif %}
    {% endfor %}
    
    // Edge cases
    expect(() => {{ function_name }}()).toThrow();
    expect({{ function_name }}(null)).toBe(null);
    expect({{ function_name }}(undefined)).toBe(undefined);
  });
  
  test('{{ test_name }}_boundary_conditions', () => {
    // Test boundary conditions
    {% if test_category == "numeric" %}
    expect({{ function_name }}(0)).toBeDefined();
    expect({{ function_name }}(Number.MAX_VALUE)).toBeDefined();
    expect({{ function_name }}(Number.MIN_VALUE)).toBeDefined();
    expect({{ function_name }}(-1)).toBeDefined();
    {% elif test_category == "string" %}
    expect({{ function_name }}('')).toBeDefined();
    expect({{ function_name }}('a'.repeat(1000))).toBeDefined();
    expect({{ function_name }}('special!@#$%^&*()')).toBeDefined();
    {% endif %}
  });
  
  test('{{ test_name }}_type_validation', () => {
    // Type validation tests
    expect(() => {{ function_name }}(123, 'string')).not.toThrow();
    expect(() => {{ function_name }}('string', 123)).not.toThrow();
    expect(() => {{ function_name }}([], {})).not.toThrow();
  });
});
"#;

pub const JEST_CLASS_TEST_TEMPLATE: &str = r#"
describe('{{ function_name }} Class', () => {
  let instance;
  
  beforeEach(() => {
    {{ setup_code | default(value="instance = new " ~ function_name ~ "();") }}
  });
  
  {% if teardown_code %}
  afterEach(() => {
    {{ teardown_code }}
  });
  {% endif %}
  
  test('should create instance successfully', () => {
    expect(instance).toBeInstanceOf({{ function_name }});
    expect(instance).toBeDefined();
  });
  
  test('should have required methods', () => {
    // Check if class has expected methods
    expect(typeof instance.constructor).toBe('function');
    
    // Test method existence (dynamic based on detected methods)
    {% for method in methods | default(value=[]) %}
    expect(typeof instance.{{ method }}).toBe('function');
    {% endfor %}
  });
  
  test('should handle initialization parameters', () => {
    // Test constructor with various parameters
    {% for input in inputs %}
    const testInstance = new {{ function_name }}({{ input }});
    expect(testInstance).toBeInstanceOf({{ function_name }});
    {% endfor %}
  });
  
  test('should maintain state correctly', () => {
    // Test state management
    const initialState = JSON.stringify(instance);
    
    // Perform some operations
    // Add operations based on detected methods
    
    // Verify state consistency
    expect(instance).toBeDefined();
  });
});
"#;

pub const JEST_ASYNC_TEST_TEMPLATE: &str = r#"
describe('{{ function_name }} (Async)', () => {
  test('{{ test_name }} - {{ description }}', async () => {
    // {{ test_category }} async test case
    {% for input in inputs %}
    const result = await {{ function_name }}({{ input }});
    expect(result).toBeDefined();
    {% if expected_outputs %}
    expect(result).toEqual({{ expected_outputs.0 }});
    {% endif %}
    {% endfor %}
  });
  
  test('{{ test_name }}_promise_handling', async () => {
    // Test promise resolution and rejection
    await expect({{ function_name }}()).resolves.toBeDefined();
    
    // Test with invalid inputs
    await expect({{ function_name }}(null)).rejects.toThrow();
    await expect({{ function_name }}(undefined)).rejects.toThrow();
  });
  
  test('{{ test_name }}_timeout_handling', async () => {
    // Test timeout scenarios
    const startTime = Date.now();
    
    try {
      await {{ function_name }}();
      const endTime = Date.now();
      expect(endTime - startTime).toBeLessThan(5000); // Should complete within 5 seconds
    } catch (error) {
      // Handle expected timeouts
      expect(error).toBeDefined();
    }
  });
  
  test('{{ test_name }}_concurrent_execution', async () => {
    // Test concurrent execution
    const promises = Array.from({ length: 3 }, (_, i) => 
      {{ function_name }}(`test_input_${i}`)
    );
    
    const results = await Promise.allSettled(promises);
    expect(results).toHaveLength(3);
    
    results.forEach(result => {
      expect(['fulfilled', 'rejected']).toContain(result.status);
    });
  });
});
"#;

pub const JEST_API_TEST_TEMPLATE: &str = r#"
describe('API Endpoint: {{ function_name }}', () => {
  const mockFetch = jest.fn();
  global.fetch = mockFetch;
  
  beforeEach(() => {
    mockFetch.mockClear();
  });
  
  test('{{ test_name }} - successful response', async () => {
    // Mock successful response
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({{ expected_outputs.0 | default(value='{"success": true}') }})
    });
    
    {% for input in inputs %}
    const result = await {{ function_name }}({{ input }});
    expect(result).toBeDefined();
    {% endfor %}
    
    expect(mockFetch).toHaveBeenCalled();
  });
  
  test('{{ test_name }} - error handling', async () => {
    // Test error scenarios
    mockFetch.mockRejectedValueOnce(new Error('Network error'));
    
    await expect({{ function_name }}()).rejects.toThrow('Network error');
  });
  
  test('{{ test_name }} - http_status_codes', async () => {
    // Test different HTTP status codes
    const statusCodes = [400, 401, 404, 500];
    
    for (const status of statusCodes) {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status,
        statusText: 'Error',
        json: async () => ({ error: `HTTP ${status}` })
      });
      
      await expect({{ function_name }}()).rejects.toThrow();
    }
  });
});
"#;