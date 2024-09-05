import java.awt.*;
import java.awt.event.*;
import java.util.ArrayList;
import javax.swing.*;

public class Display extends JPanel implements ActionListener {
    private int width;
    private int height;
    private int scale = 5;
    ArrayList<ArrayList<Integer>> grid;
    
    Timer loop;
    private int spawn = 0;
    private int color = 0;

    Display(int width, int height) {
        this.width = width;
        this.height = height;
        this.grid = new Functions().fillZeros(width, height);
        setPreferredSize(new Dimension(this.width, this.height));
        setBackground(Color.black);

        loop = new Timer(20, this);
        loop.start();
    }

    public void paintComponent(Graphics g) {
        super.paintComponent(g);
        draw(g);
    }

    public void draw(Graphics g) {
        // Draw the grid
        for (int y=0; y<grid.size(); y++) {
            for (int x=0; x<grid.get(0).size(); x++) {
                if (grid.get(y).get(x) > 0) {
                    g.setColor(new Converter().HSLToRGB(grid.get(y).get(x)/360, 1f, 0.51f)); // error
                    g.fillRect(x*scale, y*scale, scale, scale);
                }
            }
        }
    }

    @Override
    public void actionPerformed(ActionEvent e) {
        grid = new Functions().fall(grid);

        // Autonatic spawning of sand
        if (spawn > 5){
            if (color > 359){
                color = 0;
            }

            grid.get(0).set(width/scale/10, color);
            grid.get(0).set(width/scale/2, color);

            spawn = 0;
        }

        // Spawn on click

        repaint();

        spawn ++;
        color ++;
    }
}
