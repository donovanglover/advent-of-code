import { assertEquals } from "std/assert/mod.ts";

function no_time_for_a_taxi_cab_part2(s: string): number {
  let x = 0;
  let y = 0;
  let direction = 0;
  const visited: boolean[][] = [];
  const size = 1000;
  const offset = size / 2;

  for (let i = 0; i < size; i++) {
    visited[i] = new Array(size);
  }

  for (const path of s.split(", ")) {
    const amount = +path.substring(1);

    direction += path.charAt(0) === 'R' ? (direction === 3 ? -3 : 1 ) : (direction === 0 ? 3 : -1);

    const delta_x = direction % 2 === 1 ? (direction === 1 ? amount : -amount) : 0;
    const delta_y = direction % 2 === 0 ? (direction === 0 ? amount : -amount) : 0;

    for (let i = 0; i < delta_x; i++) {
      if (visited[x + i + offset][y + offset]) {
          return Math.abs(x + i) + Math.abs(y);
      } else {
        visited[x + i + offset][y + offset] = true;
      }
    }

    for (let i = 0; i < delta_y; i++) {
      if (visited[x + offset][y + i + offset]) {
          return Math.abs(x) + Math.abs(y + i);
      } else {
        visited[x + offset][y + i + offset] = true;
      }
    }

    x += delta_x;
    y += delta_y;
  }

  return 0;
}

Deno.test(function example() {
  assertEquals(no_time_for_a_taxi_cab_part2("R8, R4, R4, R8"), 4);
});

Deno.test(async function input() {
  const file = await Deno.readFile("./2016/input/01.txt");
  const input = new TextDecoder("utf-8").decode(file)

  assertEquals(no_time_for_a_taxi_cab_part2(input), 253);
});
