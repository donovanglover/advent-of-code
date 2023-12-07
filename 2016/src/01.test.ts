import { assertEquals } from "std/assert/mod.ts";

function no_time_for_a_taxi_cab(s: string): number {
  let x = 0;
  let y = 0;
  let direction = 0;

  for (const path of s.split(", ")) {
    const amount = +path.substring(1);

    direction += path.charAt(0) === 'R' ? (direction === 3 ? -3 : 1 ) : (direction === 0 ? 3 : -1);

    y += direction % 2 === 0 ? (direction === 0 ? amount : -amount) : 0;
    x += direction % 2 === 1 ? (direction === 1 ? amount : -amount) : 0;
  }

  return Math.abs(x) + Math.abs(y);
}

Deno.test(function day1_part1_example() {
  assertEquals(no_time_for_a_taxi_cab("R2, L3"), 5);
  assertEquals(no_time_for_a_taxi_cab("R2, R2, R2"), 2);
  assertEquals(no_time_for_a_taxi_cab("R5, L5, R5, R3"), 12);
});

Deno.test(async function day1_part1_input() {
  const file = await Deno.readFile("./2016/input/01.txt");
  const input = new TextDecoder("utf-8").decode(file)

  assertEquals(no_time_for_a_taxi_cab(input), 287);
});
