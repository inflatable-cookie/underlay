export function getLineStarts(text: string): number[] {
  const starts = [0];

  for (let i = 0; i < text.length; i++) {
    if (text.charCodeAt(i) === 10) starts.push(i + 1);
  }

  return starts;
}

export function getLineNumberFromIndex(lineStarts: number[], index: number): number {
  // 1-based line number
  let low = 0;
  let high = lineStarts.length - 1;

  while (low <= high) {
    const mid = (low + high) >> 1;

    if (lineStarts[mid] <= index) {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  return high + 1;
}

export function getLineText(text: string, lineStarts: number[], lineNumber: number): string {
  const start = lineStarts[lineNumber - 1] ?? 0;
  const end = lineStarts[lineNumber] ? lineStarts[lineNumber] - 1 : text.length;
  return text.slice(start, end);
}
