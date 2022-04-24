import { Maze, PixelList, Pnger } from "../pkg/index.js";

import("../pkg/index.js").catch(console.error);

let image = Pnger.new("maze(9).png");
let pixel_list = PixelList.new(image.get_bytes(), image.dimensions());
let maze = Maze.new(
    image.dimensions(),
    pixel_list,
);

let entrances = maze.find_start();

