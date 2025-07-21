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

  const [lastRequest, setLastRequest] = useState<{
    terms: string[];
    minLen: number;
    maxLen: number;
  } | null>(null);
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const termsArray = terms
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    // Validation checks for terms and lengths
    if (termsArray.length === 0) {
      alert("Please enter at least one valid term.");
      return;
    }

    if (termsArray.length > 10) {
      alert("Please enter no more than 10 terms.");
      return;
    }

    const uniqueTerms = Array.from(new Set(termsArray));
    if (uniqueTerms.length !== termsArray.length) {
      alert("Terms must be unique — please remove duplicates.");
      return;
    }

    if (minLen < 1 || maxLen < 1) {
      alert("Min Length and Max Length must both be at least 1.");
      return;
    }

    if (minLen > 10 || maxLen > 10) {
      alert("Min Length and Max Length must not exceed 10.");
      return;
    }

    if (maxLen < minLen) {
      alert("Max Length must be greater than or equal to Min Length.");
      return;
    }

    if (uniqueTerms.length < maxLen) {
      alert("The number of terms must be greater than or equal to Max Length.");
      return;
    }

    const validTerms = uniqueTerms.filter((t) => /^[a-zA-Z]+$/.test(t));
    if (validTerms.length !== uniqueTerms.length) {
      alert("Terms must only contain letters (A-Z).");
      return;
    }

    const hasVowelStart = validTerms.some((t) => /^[aeiouAEIOU]/.test(t));
    if (!hasVowelStart) {
      alert("At least one term must start with a vowel (A, E, I, O, U).");
      return;
    }

    const requestPayload = {
      terms: validTerms,
      minLen,
      maxLen,
    };

    // Check if this request matches the last one
    if (
      lastRequest &&
      JSON.stringify(lastRequest) === JSON.stringify(requestPayload)
    ) {
      alert("You already submitted this request. Please change something first.");
      return;
    }

    setLoading(true);
    setResults([]);

    try {
      const response = await fetch("/api/generate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          terms: validTerms,
          min_len: minLen,
          max_len: maxLen,
        }),
      });

      if (!response.ok) throw new Error("Failed to fetch results");

      const data: Apronym[] = await response.json();
      setResults(data);
      setLastRequest(requestPayload);  // ✅ Cache this request
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