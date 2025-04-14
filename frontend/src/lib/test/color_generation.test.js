import { expect, test } from "vitest"
import {generateColors} from "../color_generation"

test("generate 1 colors", () => {
  expect(generateColors(1)).toHaveLength(1);
})

test("generate different 2 colors", () => {
  const generatedColors = generateColors(2);
  expect(generatedColors).toHaveLength(2);
  expect(generateColors[0] == generatedColors[1]).toBeFalsy();
})

test("generate 0 colors", () => {
  try {
    generateColors(0)
  } catch (error) {
    expect(error).toBeInstanceOf(Error)
    expect(error.message).toBe("Number of colors must be greater than 0")
  }
})