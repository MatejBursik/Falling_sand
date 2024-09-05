import javax.swing.*;

class Main {
    public static void main(String[] args) {
        int width = 400;
        int height = 400;

        JFrame frame = new JFrame("Falling Sand");
        frame.setVisible(true);
        frame.setSize(width, height);
        frame.setLocationRelativeTo(null);
        frame.setResizable(false);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        Display display = new Display(width, height);
        frame.add(display);
        frame.pack();
    }
}