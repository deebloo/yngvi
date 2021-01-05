export function calculateWindChill(temp: number, windsSpeed: number): number {
  if (windsSpeed < 3) {
    return temp;
  }

  return Math.round(
    35.74 + 0.6215 * temp - 35.75 * windsSpeed ** 0.16 + 0.4275 * temp * windsSpeed ** 0.16
  );
}

export function round(value: number, decimals: number) {
  return Number(Math.round(Number(value + 'e' + decimals)) + 'e-' + decimals);
}
