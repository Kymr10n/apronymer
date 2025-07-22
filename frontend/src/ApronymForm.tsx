import { useState } from "react";
import { FiDownload, FiCopy } from "react-icons/fi";

interface Apronym {
  text: string;
  terms: string[];
}

export default function ApronymForm() {
  const [terms, setTerms] = useState("");
  const [fragLen, setFragLen] = useState(1);
  const [minLen, setMinLen] = useState(2);
  const [maxLen, setMaxLen] = useState(4);
  const [results, setResults] = useState<Apronym[]>([]);
  const [loading, setLoading] = useState(false);

  const [lastRequest, setLastRequest] = useState<{
    terms: string[];
    fragLen: number;
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

    if (fragLen < 1 || fragLen > 3) {
      alert("Fragment Length must be between 1 and 3.");
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

    if (maxLen > uniqueTerms.length) {
      alert("Max Length cannot exceed the number of terms provided.");
      return;
    }

    const validTerms = uniqueTerms.filter((t) => /^[a-zA-Z]+$/.test(t));
    if (validTerms.length !== uniqueTerms.length) {
      alert("Terms must only contain letters (A-Z).");
      return;
    }

    const maxPossibleLength = fragLen * validTerms.length;
    if (maxLen > maxPossibleLength) {
      alert(`Max Length cannot exceed ${maxPossibleLength} (Fragment Length × Number of Terms).`);
      return;
    }

    const hasVowelStart = validTerms.some((t) => /^[aeiouAEIOU]/.test(t));
    if (!hasVowelStart) {
      alert("At least one term must start with a vowel (A, E, I, O, U).");
      return;
    }

    const requestPayload = {
      terms: validTerms,
      fragLen,
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
          frag_len: fragLen,
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
          <label className="block font-semibold">Fragment Length:</label>
          <input
            type="number"
            value={fragLen}
            onChange={(e) => setFragLen(Number(e.target.value))}
            className="w-full border rounded p-2"
            min="1"
            max="3"
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
            min="1"
            max={Math.min(10, fragLen * terms.split(",").filter(t => t.trim().length > 0).length)}
          />
          <small className="text-gray-500">
            Maximum possible: {fragLen * terms.split(",").filter(t => t.trim().length > 0).length}
          </small>
        </div>
        <button
          type="submit"
          className="w-full bg-blue-500 text-white py-2 rounded hover:bg-blue-600"
        >
          {loading ? "Generating..." : "Generate"}
        </button>
      </form>

      <div className="mt-6">
        <div className="flex items-center gap-2 mb-2">
          <h2 className="text-xl font-bold">Results:</h2>
          <button
            type="button"
            title="Copy results"
            className="p-2 rounded hover:bg-gray-200"
            onClick={() => {
              if (results.length === 0) return;
              const text = results.map(r => `${r.text}: ${r.terms.join(", ")}`).join("\n");
              navigator.clipboard.writeText(text);
            }}
            disabled={results.length === 0}
          >
            <FiCopy size={20} />
          </button>
          <button
            type="button"
            title="Download results"
            className="p-2 rounded hover:bg-gray-200"
            onClick={() => {
              if (results.length === 0) return;
              const text = results.map(r => `${r.text}: ${r.terms.join(", ")}`).join("\n");
              const blob = new Blob([text], { type: "text/plain" });
              const url = URL.createObjectURL(blob);
              const a = document.createElement("a");
              a.href = url;
              a.download = "apronyms.txt";
              document.body.appendChild(a);
              a.click();
              document.body.removeChild(a);
              URL.revokeObjectURL(url);
            }}
            disabled={results.length === 0}
          >
            <FiDownload size={20} />
          </button>
        </div>
        {results.length === 0 && !loading && <p>No results yet.</p>}
        <ul className="list-disc pl-5 space-y-1">
          {results.map((apronym, idx) => (
            <li key={idx}>
              <span className="font-semibold">{apronym.text}</span>: {apronym.terms.join(", ")}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}