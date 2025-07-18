import { useState } from "react";

interface Apronym {
  name: string;
  terms: string[];
}

export default function ApronymForm() {
  const [terms, setTerms] = useState("");
  const [minLen, setMinLen] = useState(2);
  const [maxLen, setMaxLen] = useState(4);
  const [results, setResults] = useState<Apronym[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const termsArray = terms
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    if (termsArray.length === 0) return;

    setLoading(true);
    setResults([]);

    try {
      const response = await fetch("/api/generate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          terms: termsArray,
          min_len: minLen,
          max_len: maxLen,
        }),
      });

      if (!response.ok) throw new Error("Failed to fetch results");

      const data: Apronym[] = await response.json();
      setResults(data);
    } catch (err) {
      console.error(err);
      alert("An error occurred. Check console for details.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-md mx-auto">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block font-semibold">Terms (comma separated):</label>
          <input
            type="text"
            value={terms}
            onChange={(e) => setTerms(e.target.value)}
            className="w-full border rounded p-2"
          />
        </div>
        <div>
          <label className="block font-semibold">Min Length:</label>
          <input
            type="number"
            value={minLen}
            onChange={(e) => setMinLen(Number(e.target.value))}
            className="w-full border rounded p-2"
          />
        </div>
        <div>
          <label className="block font-semibold">Max Length:</label>
          <input
            type="number"
            value={maxLen}
            onChange={(e) => setMaxLen(Number(e.target.value))}
            className="w-full border rounded p-2"
          />
        </div>
        <button
          type="submit"
          className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600"
        >
          {loading ? "Generating..." : "Generate"}
        </button>
      </form>

      <div className="mt-6">
        <h2 className="text-xl font-bold">Results:</h2>
        {results.length === 0 && !loading && <p>No results yet.</p>}
        <ul className="list-disc pl-5 space-y-1">
          {results.map((apronym, idx) => (
            <li key={idx}>
              <span className="font-semibold">{apronym.name}</span>: {apronym.terms.join(", ")}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}