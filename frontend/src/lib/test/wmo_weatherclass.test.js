import {wmo_code_to_weather} from "../wmo_to_weatherclass"
import { test, expect } from 'vitest';

test("Clear Class", () => {
  expect(wmo_code_to_weather(0)).toBe("Clear");
  expect(wmo_code_to_weather(1)).toBe("Clear");
})

test("Cloudy Class", () => {
  expect(wmo_code_to_weather(2)).toBe("Cloudy");
  expect(wmo_code_to_weather(3)).toBe("Cloudy");
})

test("Low Visibility Class", () => {
  expect(wmo_code_to_weather(45)).toBe("Low Visibility")
  expect(wmo_code_to_weather(48)).toBe("Low Visibility")
})

test("Rain Class", () => {
  expect(wmo_code_to_weather(51)).toBe("Rain")
  expect(wmo_code_to_weather(99)).toBe("Rain")
})

test("Unknown Class", () => {
  expect(wmo_code_to_weather(4)).toBe("Unknown")
  expect(wmo_code_to_weather(69)).toBe("Unknown")
})
