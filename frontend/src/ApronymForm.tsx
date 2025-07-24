import { useState } from "react";
import { FormField } from "./FormField";
import { ResultsList } from "./ResultsList";
import type { Apronym } from "./ResultsList";
import { useApronymValidation } from "./useApronymValidation";
import { API_BASE_URL, API_KEY } from "./config";

/**
 * ApronymForm: Main form for generating apronynms.
 */
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
  const validate = useApronymValidation();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const { validTerms, error } = validate(terms, fragLen, minLen, maxLen);
    if (error) {
      alert(error);
      return;
    }
    // Fragment length validation is now handled in the validation hook
    const requestPayload = {
      terms: validTerms,
      fragLen,
      minLen,
      maxLen,
    };
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
      const response = await fetch(`${API_BASE_URL}/api/generate`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-api-key": API_KEY
        },
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
      setLastRequest(requestPayload);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "An unexpected error occurred";
      alert(`Failed to generate apronyms: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-md mx-auto">
      <form onSubmit={handleSubmit} className="space-y-4">
        <FormField
          label="Terms (comma separated):"
          type="text"
          value={terms}
          onChange={e => setTerms(e.target.value)}
          inputMode="text"
          autoComplete="off"
        />
        <FormField
          label="Fragment Length:"
          type="number"
          value={fragLen}
          onChange={e => {
            const val = e.target.value === "" ? 1 : Number(e.target.value);
            setFragLen(val);
          }}
          min={1}
          max={3}
          inputMode="numeric"
        />
        <FormField
          label="Min Length:"
          type="number"
          value={minLen}
          onChange={e => {
            const val = e.target.value === "" ? 1 : Number(e.target.value);
            setMinLen(val);
          }}
          min={1}
          inputMode="numeric"
        />
        <FormField
          label="Max Length:"
          type="number"
          value={maxLen}
          onChange={e => {
            const val = e.target.value === "" ? 1 : Number(e.target.value);
            setMaxLen(val);
          }}
          min={1}
          max={Math.min(10, fragLen * terms.split(",").filter(t => t.trim().length > 0).length)}
          inputMode="numeric"
        />
        <small className="text-gray-500 block mt-1">
          Maximum possible: {fragLen * terms.split(",").filter(t => t.trim().length > 0).length}
        </small>
        <button
          type="submit"
          className="w-full bg-blue-500 text-white py-3 rounded text-base sm:text-lg hover:bg-blue-600"
          style={{ minHeight: 44 }}
        >
          {loading ? "Generating..." : "Generate"}
        </button>
      </form>
      <ResultsList results={results} loading={loading} />
    </div>
  );
}