import { test, expect } from 'vitest';
import { generateTimeOptions, timeToRFC3339 } from "../time_utils"
import { randomInt } from 'node:crypto';

test("generateTimeOptions", () => {
  const generatedOptions = generateTimeOptions();
  expect(generatedOptions).toHaveLength(24 * 2);
  // Check Boundaries
  expect(generatedOptions[0]).toBe("00:00");
  expect(generatedOptions[47]).toBe("23:30");
  expect(generatedOptions.includes("24:00")).toBeFalsy();
})

test("TimeToRFC3339 at 00:01", () => {
  const now = new Date()
  const month =(now.getMonth()+1).toString().padStart(2, '0')
  const day =now.getDate().toString().padStart(2, '0')
  expect(timeToRFC3339("00:01")).toBe(
    `${now.getFullYear()}-${month}-${day}T00:30:00+07:00`);
})

test("TimeToRFC3339 at 00:45", () => {
  const now = new Date()
  const month =(now.getMonth()+1).toString().padStart(2, '0')
  const day =now.getDate().toString().padStart(2, '0')
  expect(timeToRFC3339("00:45")).toBe(
    `${now.getFullYear()}-${month}-${day}T01:00:00+07:00`);
})

test("TimeToRFC3339 at noon", () => {
  const now = new Date()
  const month =(now.getMonth()+1).toString().padStart(2, '0')
  const day =now.getDate().toString().padStart(2, '0')
  expect(timeToRFC3339("12:00")).toBe(
    `${now.getFullYear()}-${month}-${day}T12:00:00+07:00`);
})

test("TimeToRFC3339 with negative hours",() => {
  try{
    timeToRFC3339("-1:00")
  } catch(error) {
    expect(error.message).toBe('Invalid time, Time should be in range of 00:00 and 23:59');
  }
})

test("TimeToRFC3339 with negative minutes",() => {
  try{
    timeToRFC3339("00:-1")
  } catch(error) {
    expect(error.message).toBe('Invalid time, Time should be in range of 00:00 and 23:59');
  }
})

test("TimeToRFC3339 with overflow hours",() => {
  try{
    timeToRFC3339("24:00")
  } catch(error) {
    expect(error.message).toBe('Invalid time, Time should be in range of 00:00 and 23:59');
  }
})

test("TimeToRFC3339 with overflow minute",() => {
  try{
    timeToRFC3339("23:60")
  } catch(error) {
    expect(error.message).toBe('Invalid time, Time should be in range of 00:00 and 23:59');
  }
})