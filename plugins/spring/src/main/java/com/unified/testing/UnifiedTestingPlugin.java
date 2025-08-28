package com.unified.testing;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.progress.ProgressIndicator;
import com.intellij.openapi.progress.ProgressManager;
import com.intellij.openapi.progress.Task;
import com.intellij.openapi.application.ApplicationManager;
import com.intellij.openapi.command.WriteCommandAction;
import com.intellij.openapi.fileEditor.FileDocumentManager;
import com.intellij.openapi.diagnostic.Logger;

import org.jetbrains.annotations.NotNull;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

/**
 * Main plugin class for Unified Testing Framework integration with IntelliJ IDEA / Spring Tool Suite
 */
public class UnifiedTestingPlugin {
    private static final Logger LOG = Logger.getInstance(UnifiedTestingPlugin.class);
    private static final String PLUGIN_NAME = "Unified Testing Framework";
    
    /**
     * Action to generate tests for the currently open file
     */
    public static class GenerateTestsAction extends AnAction {
        
        public GenerateTestsAction() {
            super("Generate Tests", "Generate unit tests for the current file", null);
        }
        
        @Override
        public void actionPerformed(@NotNull AnActionEvent e) {
            Project project = e.getProject();
            if (project == null) {
                Messages.showErrorDialog("No project is open", PLUGIN_NAME);
                return;
            }
            
            VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
            if (file == null) {
                Messages.showErrorDialog("No file is selected", PLUGIN_NAME);
                return;
            }
            
            // Run the test generation in a background task
            ProgressManager.getInstance().run(new Task.Backgroundable(project, "Generating Tests...", true) {
                @Override
                public void run(@NotNull ProgressIndicator indicator) {
                    try {
                        indicator.setIndeterminate(false);
                        indicator.setFraction(0.0);
                        indicator.setText("Analyzing code patterns...");
                        
                        String filePath = file.getPath();
                        String result = executeUnifiedTesting("generate", filePath);
                        
                        indicator.setFraction(1.0);
                        indicator.setText("Tests generated successfully");
                        
                        ApplicationManager.getApplication().invokeLater(() -> {
                            Messages.showInfoMessage(project, 
                                "Tests generated successfully:\n" + result, 
                                PLUGIN_NAME);
                        });
                        
                    } catch (Exception ex) {
                        LOG.error("Failed to generate tests", ex);
                        ApplicationManager.getApplication().invokeLater(() -> {
                            Messages.showErrorDialog(project, 
                                "Failed to generate tests: " + ex.getMessage(), 
                                PLUGIN_NAME);
                        });
                    }
                }
            });
        }
        
        @Override
        public void update(@NotNull AnActionEvent e) {
            VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
            boolean enabled = file != null && isSupported(file);
            e.getPresentation().setEnabled(enabled);
        }
    }
    
    /**
     * Action to analyze patterns in the currently open file
     */
    public static class AnalyzeFileAction extends AnAction {
        
        public AnalyzeFileAction() {
            super("Analyze Patterns", "Analyze testable patterns in the current file", null);
        }
        
        @Override
        public void actionPerformed(@NotNull AnActionEvent e) {
            Project project = e.getProject();
            if (project == null) {
                Messages.showErrorDialog("No project is open", PLUGIN_NAME);
                return;
            }
            
            VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
            if (file == null) {
                Messages.showErrorDialog("No file is selected", PLUGIN_NAME);
                return;
            }
            
            // Run the analysis in a background task
            ProgressManager.getInstance().run(new Task.Backgroundable(project, "Analyzing File...", true) {
                @Override
                public void run(@NotNull ProgressIndicator indicator) {
                    try {
                        indicator.setIndeterminate(false);
                        indicator.setFraction(0.0);
                        indicator.setText("Analyzing code patterns...");
                        
                        String filePath = file.getPath();
                        String result = executeUnifiedTesting("analyze", filePath);
                        
                        indicator.setFraction(1.0);
                        indicator.setText("Analysis completed");
                        
                        ApplicationManager.getApplication().invokeLater(() -> {
                            Messages.showInfoMessage(project, 
                                "Analysis results:\n" + result, 
                                PLUGIN_NAME);
                        });
                        
                    } catch (Exception ex) {
                        LOG.error("Failed to analyze file", ex);
                        ApplicationManager.getApplication().invokeLater(() -> {
                            Messages.showErrorDialog(project, 
                                "Failed to analyze file: " + ex.getMessage(), 
                                PLUGIN_NAME);
                        });
                    }
                }
            });
        }
        
        @Override
        public void update(@NotNull AnActionEvent e) {
            VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
            boolean enabled = file != null && isSupported(file);
            e.getPresentation().setEnabled(enabled);
        }
    }
    
    /**
     * Check if the file type is supported by the Unified Testing Framework
     */
    private static boolean isSupported(VirtualFile file) {
        String extension = file.getExtension();
        return extension != null && (
            extension.equals("js") || 
            extension.equals("jsx") || 
            extension.equals("ts") || 
            extension.equals("tsx") || 
            extension.equals("py") || 
            extension.equals("rs")
        );
    }
    
    /**
     * Execute the unified-testing binary with the given command and file path
     */
    private static String executeUnifiedTesting(String command, String filePath) throws IOException, InterruptedException {
        List<String> commandLine = new ArrayList<>();
        
        // Try to find the unified-testing binary
        String binaryPath = findUnifiedTestingBinary();
        commandLine.add(binaryPath);
        commandLine.add(command);
        commandLine.add(filePath);
        
        ProcessBuilder processBuilder = new ProcessBuilder(commandLine);
        processBuilder.redirectErrorStream(true);
        
        Process process = processBuilder.start();
        
        StringBuilder output = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
            String line;
            while ((line = reader.readLine()) != null) {
                output.append(line).append("\n");
            }
        }
        
        int exitCode = process.waitFor();
        if (exitCode != 0) {
            throw new RuntimeException("Command failed with exit code: " + exitCode);
        }
        
        return output.toString().trim();
    }
    
    /**
     * Find the unified-testing binary in common locations
     */
    private static String findUnifiedTestingBinary() {
        String[] possiblePaths = {
            "unified-testing",
            "./target/release/unified-testing",
            "./target/debug/unified-testing",
            System.getProperty("user.home") + "/.cargo/bin/unified-testing"
        };
        
        for (String path : possiblePaths) {
            Path binaryPath = Paths.get(path);
            if (Files.exists(binaryPath) && Files.isExecutable(binaryPath)) {
                return path;
            }
        }
        
        // Default to system PATH
        return "unified-testing";
    }
}