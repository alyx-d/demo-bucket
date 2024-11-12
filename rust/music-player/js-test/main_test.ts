import { assertEquals } from "https://deno.land/std@0.103.0/testing/asserts.ts";

function add(a: number, b: number): number {
    return a + b;
}

Deno.test("test function", () => {
    assertEquals(3, add(1, 2));
    assertEquals(4, add(1, 3));
});
