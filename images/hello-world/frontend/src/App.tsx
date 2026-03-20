import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

function App() {
  const [a, setA] = useState("0");
  const [b, setB] = useState("0");
  const [result, setResult] = useState("");

  async function calc(op: string) {
    setResult("Calculating...");
    try {
      const res = await fetch("/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ cmd: ["/calculate.sh", a, b, op] }),
      });
      const text = await res.text();
      const data = JSON.parse(text);
      if (data.error) {
        setResult("Error: " + data.error);
      } else {
        setResult(`Result: ${data.result}`);
      }
    } catch (e) {
      setResult("Error: " + e);
    }
  }

  async function benchmark() {
    setResult("Benchmarking...");
    const duration = 3000;
    const start = performance.now();
    let count = 0;
    while (performance.now() - start < duration) {
      await fetch("/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ cmd: ["/calculate.sh", "2", "3", "add"] }),
      });
      count++;
      setResult(`Benchmarking... ${count} calls`);
    }
    const elapsed = (performance.now() - start) / 1000;
    const rps = (count / elapsed).toFixed(1);
    const avg = ((elapsed / count) * 1000).toFixed(1);
    setResult(
      `${count} calls in ${elapsed.toFixed(1)}s — ${rps} req/s, ${avg}ms avg`
    );
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center gap-6">
      <h1 className="text-5xl font-bold">Hello from Potato!</h1>
      <div className="flex gap-2 items-center">
        <Input
          type="number"
          value={a}
          onChange={(e) => setA(e.target.value)}
          className="w-24 text-center"
        />
        <Input
          type="number"
          value={b}
          onChange={(e) => setB(e.target.value)}
          className="w-24 text-center"
        />
      </div>
      <div className="flex gap-2">
        <Button onClick={() => calc("add")}>Add</Button>
        <Button onClick={() => calc("multiply")}>Multiply</Button>
        <Button variant="secondary" onClick={benchmark}>
          Benchmark
        </Button>
      </div>
      {result && <p className="text-2xl">{result}</p>}
    </div>
  );
}

export default App;
