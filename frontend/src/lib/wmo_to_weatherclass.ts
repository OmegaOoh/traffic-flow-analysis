export function wmo_code_to_weather(weatherCode: number): string {
  if ([0,1].includes(weatherCode)) return "Clear";
  if ([2,3].includes(weatherCode)) return "Cloudy";
  if ([45, 48].includes(weatherCode)) return "Low Visibility";
  if ([51,53,55,56,57,58,59,60,61,63,65,66,67,71,73,75,77,80,81,82,85,86,95,96,99].includes(weatherCode)) 
    return "Rain";
  else return "Unknown";
};