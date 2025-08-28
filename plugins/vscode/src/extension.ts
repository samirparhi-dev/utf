import * as vscode from 'vscode';
import * as path from 'path';
import { spawn, ChildProcess } from 'child_process';

export function activate(context: vscode.ExtensionContext) {
    console.log('Unified Testing Extension is now active!');

    // Register commands
    const generateTestsCommand = vscode.commands.registerCommand('unified-testing.generateTests', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No file is currently open');
            return;
        }

        const filePath = editor.document.uri.fsPath;
        const outputChannel = vscode.window.createOutputChannel('Unified Testing');
        
        try {
            outputChannel.show();
            outputChannel.appendLine(`Generating tests for: ${filePath}`);
            
            const result = await executeUnifiedTesting(['generate', filePath]);
            outputChannel.appendLine('Tests generated successfully:');
            outputChannel.appendLine(result);
            
            vscode.window.showInformationMessage('Tests generated successfully!');
        } catch (error) {
            outputChannel.appendLine(`Error: ${error}`);
            vscode.window.showErrorMessage(`Failed to generate tests: ${error}`);
        }
    });

    const analyzeFileCommand = vscode.commands.registerCommand('unified-testing.analyzeFile', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No file is currently open');
            return;
        }

        const filePath = editor.document.uri.fsPath;
        const outputChannel = vscode.window.createOutputChannel('Unified Testing');
        
        try {
            outputChannel.show();
            outputChannel.appendLine(`Analyzing patterns in: ${filePath}`);
            
            const result = await executeUnifiedTesting(['analyze', filePath]);
            outputChannel.appendLine('Analysis results:');
            outputChannel.appendLine(result);
            
            vscode.window.showInformationMessage('File analysis completed!');
        } catch (error) {
            outputChannel.appendLine(`Error: ${error}`);
            vscode.window.showErrorMessage(`Failed to analyze file: ${error}`);
        }
    });

    // Register code lens provider for test generation
    const codeLensProvider = new UnifiedTestingCodeLensProvider();
    const codeLensDisposable = vscode.languages.registerCodeLensProvider(
        [
            { language: 'javascript' },
            { language: 'typescript' },
            { language: 'python' },
            { language: 'rust' }
        ],
        codeLensProvider
    );

    // Register hover provider for pattern insights
    const hoverProvider = new UnifiedTestingHoverProvider();
    const hoverDisposable = vscode.languages.registerHoverProvider(
        [
            { language: 'javascript' },
            { language: 'typescript' },
            { language: 'python' },
            { language: 'rust' }
        ],
        hoverProvider
    );

    context.subscriptions.push(
        generateTestsCommand,
        analyzeFileCommand,
        codeLensDisposable,
        hoverDisposable
    );
}

class UnifiedTestingCodeLensProvider implements vscode.CodeLensProvider {
    async provideCodeLenses(document: vscode.TextDocument): Promise<vscode.CodeLens[]> {
        const codeLenses: vscode.CodeLens[] = [];
        const text = document.getText();
        
        // Simple pattern detection for function declarations
        const functionRegex = /function\s+(\w+)\s*\(([^)]*)\)/g;
        const pyFunctionRegex = /def\s+(\w+)\s*\(([^)]*)\):/g;
        const rustFunctionRegex = /(pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)/g;
        
        let match;
        
        // JavaScript/TypeScript functions
        while ((match = functionRegex.exec(text)) !== null) {
            const line = document.positionAt(match.index).line;
            const range = new vscode.Range(line, 0, line, match[0].length);
            
            codeLenses.push(new vscode.CodeLens(range, {
                title: "🧪 Generate Tests",
                command: "unified-testing.generateTests"
            }));
        }
        
        // Python functions
        while ((match = pyFunctionRegex.exec(text)) !== null) {
            const line = document.positionAt(match.index).line;
            const range = new vscode.Range(line, 0, line, match[0].length);
            
            codeLenses.push(new vscode.CodeLens(range, {
                title: "🧪 Generate Tests",
                command: "unified-testing.generateTests"
            }));
        }
        
        // Rust functions
        while ((match = rustFunctionRegex.exec(text)) !== null) {
            const line = document.positionAt(match.index).line;
            const range = new vscode.Range(line, 0, line, match[0].length);
            
            codeLenses.push(new vscode.CodeLens(range, {
                title: "🧪 Generate Tests",
                command: "unified-testing.generateTests"
            }));
        }
        
        return codeLenses;
    }
}

class UnifiedTestingHoverProvider implements vscode.HoverProvider {
    async provideHover(document: vscode.TextDocument, position: vscode.Position): Promise<vscode.Hover | undefined> {
        const word = document.getWordRangeAtPosition(position);
        if (!word) return;
        
        const text = document.getText(word);
        const line = document.lineAt(position.line).text;
        
        // Check if hovering over a function
        if (line.includes('function ') || line.includes('def ') || line.includes('fn ')) {
            const markdown = new vscode.MarkdownString();
            markdown.appendMarkdown('**Unified Testing Framework**\n\n');
            markdown.appendMarkdown('This function can be automatically tested.\n\n');
            markdown.appendMarkdown('[Generate Tests](command:unified-testing.generateTests) | ');
            markdown.appendMarkdown('[Analyze Patterns](command:unified-testing.analyzeFile)');
            markdown.isTrusted = true;
            
            return new vscode.Hover(markdown);
        }
        
        // Check if hovering over email input patterns
        if (line.includes('type="email"') || line.includes("type='email'")) {
            const markdown = new vscode.MarkdownString();
            markdown.appendMarkdown('**Email Field Detected**\n\n');
            markdown.appendMarkdown('Unified Testing can generate validation tests for this email field.\n\n');
            markdown.appendMarkdown('[Generate Tests](command:unified-testing.generateTests)');
            markdown.isTrusted = true;
            
            return new vscode.Hover(markdown);
        }
        
        return undefined;
    }
}

async function executeUnifiedTesting(args: string[]): Promise<string> {
    return new Promise((resolve, reject) => {
        // Try to find unified-testing binary in common locations
        const possiblePaths = [
            'unified-testing',
            './target/release/unified-testing',
            './target/debug/unified-testing',
            path.join(process.env.HOME || '', '.cargo', 'bin', 'unified-testing')
        ];
        
        let binaryPath = 'unified-testing';
        
        // Use the first available path
        for (const testPath of possiblePaths) {
            try {
                const testProcess = spawn(testPath, ['--help'], { stdio: 'ignore' });
                testProcess.on('close', (code) => {
                    if (code === 0) {
                        binaryPath = testPath;
                    }
                });
            } catch (e) {
                // Continue to next path
            }
        }
        
        const process = spawn(binaryPath, args, {
            stdio: ['pipe', 'pipe', 'pipe']
        });
        
        let stdout = '';
        let stderr = '';
        
        process.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        process.on('close', (code) => {
            if (code === 0) {
                resolve(stdout);
            } else {
                reject(new Error(`Command failed with code ${code}: ${stderr}`));
            }
        });
        
        process.on('error', (error) => {
            reject(new Error(`Failed to execute unified-testing: ${error.message}`));
        });
    });
}

export function deactivate() {
    console.log('Unified Testing Extension deactivated');
}