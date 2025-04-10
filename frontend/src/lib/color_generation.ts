export function generateColors(numColors: number) {
  const colors = [];
  for (let i = 0; i < numColors; i++) {
    const hue = Math.floor((360 / numColors) * i);
    colors.push(`hsl(${hue}, 60%, 60%)`);
  }
  return colors;
}