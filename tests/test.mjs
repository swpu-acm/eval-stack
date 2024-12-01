let input = '';

process.stdin.on('data', (data) => {
  input += data;
});

process.stdin.on('end', () => {
  const numbers = input.split('\n')[0].split(" ").map((num) => parseInt(num));
  const result = numbers.reduce((acc, num) => acc + num, 0);
  console.log(result);
  console.log(result);
});
