using System;
using System.Collections.Generic;

namespace ConsoleApp1
{
    class Cudoviste
    {
        private static void Main()
        {
            string[] inputString = Console.ReadLine().Split(" ");
            int x = 0,y = 0, cars = 0;
            
            int rows = Convert.ToInt32(inputString[0]), cols = Convert.ToInt32(inputString[1]);

            var grid = new List<string>();
            for (int i = 0; i < rows; i++)
            {
                grid.Add(Console.ReadLine());
            }

            var parkingSpots = new List<int>(new int[5]);

            for (int i = 0; i < rows - 1; i++)
            {
                for (int j = 0; j < cols - 1; j++)
                {
                    x = i;
                    y = j;
                    cars = CheckGrid(grid, x, y);
                    if (cars >= 0)
                    {
                        parkingSpots[cars]++;
                    }
                }
            }

            for (int i = 0; i < 5; i++)
            {
                Console.WriteLine(parkingSpots[i]);
            }
        }

        private static int CheckGrid(List<string> grid, int x, int y)
        {
            int cars = 0;
            for (int i = x; i < x+2; i++)
            {
                for (int j = y; j < y+2; j++)
                {
                    switch (grid[i][j])
                    {
                        case '#': // building
                            return -1;
                        case 'X': // parked car
                            cars++;
                            break;
                    }
                }
            }
            return cars;
        }
    }
}