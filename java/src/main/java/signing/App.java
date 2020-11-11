/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package signing;

import picocli.CommandLine;

public class App {
    public static void main(String[] args) {
        CommandLine commandLine = new CommandLine(new RequestSigner());

        if (args.length == 0) {
            commandLine.usage(System.out);
            return;
        }

        int exitCode = commandLine.execute(args);
        System.exit(exitCode);
    }
}
