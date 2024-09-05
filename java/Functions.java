import java.util.ArrayList;
import java.util.Random;

public class Functions {
    // Fill the grid with zeros
    public ArrayList<ArrayList<Integer>> fillZeros(int w, int h) {
        ArrayList<ArrayList<Integer>> grid = new ArrayList<>();
        ArrayList<Integer> hold;

        for (int y=0; y<h; y++) {
            hold = new ArrayList<>();
            for (int x=0; x<w; x++) {
                hold.add(0);
            }
            grid.add(hold);
        }
        return grid;
    }

    // Update the grid as if it is falling
    public ArrayList<ArrayList<Integer>> fall(ArrayList<ArrayList<Integer>> grid) {
        ArrayList<ArrayList<Integer>> newGrid = fillZeros(grid.get(0).size(), grid.size());

        int[] choices = {1,-1};
        Random random = new Random();
        int dir = choices[random.nextInt(choices.length)];

        for (int y=0; y<grid.size(); y++) {
            for (int x=0; x<grid.get(0).size(); x++) {
                if (grid.get(y).get(x) > 0) {
                    if (y+1 < grid.size()){
                        if (grid.get(y+1).get(x) == 0) {
                            newGrid.get(y+1).set(x, grid.get(y).get(x));
                        } else if (x+1 < grid.get(0).size() && dir == 1) {
                            if (grid.get(y+1).get(x+1) == 0) {
                                newGrid.get(y+1).set(x+1, grid.get(y).get(x));
                            } else {
                                newGrid.get(y).set(x, grid.get(y).get(x));
                            }
                        } else if (x-1 >= 0 && dir == -1) {
                            if (grid.get(y+1).get(x-1) == 0) {
                                newGrid.get(y+1).set(x-1, grid.get(y).get(x));
                            } else {
                                newGrid.get(y).set(x, grid.get(y).get(x));
                            }
                        } else {
                            newGrid.get(y).set(x, grid.get(y).get(x));
                        }
                    } else {
                        newGrid.get(y).set(x, grid.get(y).get(x));
                    }
                }
            }
        }

        return newGrid;
    }
}
