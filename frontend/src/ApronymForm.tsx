import { useState, useEffect } from "react";
import { FormField } from "./FormField";
import { ResultsList } from "./ResultsList";
import type { Apronym } from "./ResultsList";
import { useApronymValidation } from "./useApronymValidation";
import { useClientRateLimit } from "./useClientRateLimit";
import { API_BASE_URL, API_KEY } from "./config";
import { SliderField } from "./SliderField";

/**
 * ApronymForm: Main form for generating apronynms.
 */
export default function ApronymForm() {
  const [terms, setTerms] = useState("");
  const [fragLen, setFragLen] = useState(1);
  const [minLen, setMinLen] = useState(3);
  const [maxLen, setMaxLen] = useState(3);
  const [results, setResults] = useState<Apronym[]>([]);
  const [loading, setLoading] = useState(false);
  const [lastRequest, setLastRequest] = useState<{
    terms: string[];
    fragLen: number;
    minLen: number;
    maxLen: number;
  } | null>(null);
  
  const validate = useApronymValidation();
  const rateLimit = useClientRateLimit({
    maxRequests: 5,        // 5 requests per minute (more restrictive than backend)
    windowMs: 60 * 1000,   // 1 minute window
    cooldownMs: 3 * 1000   // 3 second cooldown between requests
  });

  // Start auto-updating rate limit state
  useEffect(() => {
    const cleanup = rateLimit.startAutoUpdate();
    return cleanup;
  }, [rateLimit]);

  // Calculate dynamic slider ranges
  const termsArr = terms.split(",").map(t => t.trim()).filter(Boolean);
  const numTerms = termsArr.length || 1;
  const minTermLength = termsArr.reduce((min, t) => t.length < min ? t.length : min, 99);
  const fragLenMax = minTermLength > 0 ? Math.min(3, minTermLength) : 3;
  const maxLenMax = numTerms;
  const minLenMax = Math.max(1, Math.min(maxLen, 10));

  // Clamp values if user input goes out of bounds
  if (fragLen > fragLenMax) setFragLen(fragLenMax);
  if (maxLen > maxLenMax) setMaxLen(maxLenMax);
  if (minLen > minLenMax) setMinLen(minLenMax);
  if (minLen < 1) setMinLen(1);
  if (fragLen < 1) setFragLen(1);
  if (maxLen < 1) setMaxLen(1);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Client-side rate limiting check
    if (!rateLimit.attemptRequest()) {
      alert(rateLimit.lastBlockedReason || "Please wait before submitting again");
      return;
    }
    
    const { validTerms, error } = validate(terms, fragLen, minLen, maxLen);
    if (error) {
      alert(error);
      return;
    }
    
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
      
      if (response.status === 429) {
        throw new Error("Rate limit exceeded. Please wait a moment before trying again.");
      }
      
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

  // Feedback button handler
  const handleFeedback = () => {
    const title = encodeURIComponent("Feedback: Apronym Generator");
    const body = encodeURIComponent(
      `**Describe your feedback or issue:**\n\n\n---\n**Form State:**\n- Terms: ${terms}\n- Fragment Length: ${fragLen}\n- Min Length: ${minLen}\n- Max Length: ${maxLen}`
    );
    const url = `https://github.com/Kymr10n/apronymer/issues/new?title=${title}&body=${body}`;
    window.open(url, "_blank");
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
        <SliderField
          label="Fragment Length:"
          value={fragLen}
          min={1}
          max={fragLenMax}
          onChange={setFragLen}
        />
        <SliderField
          label="Max Length:"
          value={maxLen}
          min={1}
          max={maxLenMax}
          onChange={setMaxLen}
        />
        <SliderField
          label="Min Length:"
          value={minLen}
          min={1}
          max={minLenMax}
          onChange={setMinLen}
        />
        <button
          type="submit"
          disabled={loading || !rateLimit.canSubmit}
          className={`w-full py-3 rounded text-base sm:text-lg transition-colors ${
            loading || !rateLimit.canSubmit
              ? "bg-gray-400 text-gray-700 cursor-not-allowed"
              : "bg-blue-500 text-white hover:bg-blue-600"
          }`}
          style={{ minHeight: 44 }}
        >
          {loading 
            ? "Generating..." 
            : !rateLimit.canSubmit 
              ? `Wait ${Math.ceil(rateLimit.timeUntilReset / 1000)}s` 
              : `Generate (${rateLimit.requestsRemaining} left)`
          }
        </button>
        {!rateLimit.canSubmit && rateLimit.lastBlockedReason && (
          <p className="text-red-600 text-sm text-center mt-2">
            {rateLimit.lastBlockedReason}
          </p>
        )}
      </form>
      
      {/* Rate limit info */}
      <div className="mt-4 p-3 bg-blue-50 rounded-lg border border-blue-200">
        <div className="text-xs text-blue-800">
          <div className="font-medium">Rate Limit Status:</div>
          <div className="mt-1">
            Requests remaining: <span className="font-mono">{rateLimit.requestsRemaining}/{rateLimit.config.maxRequests}</span>
          </div>
          {rateLimit.timeUntilReset > 0 && (
            <div className="mt-1">
              Reset in: <span className="font-mono">{Math.ceil(rateLimit.timeUntilReset / 1000)}s</span>
            </div>
          )}
        </div>
      </div>
      
      <ResultsList results={results} loading={loading} />
      <button
        type="button"
        className="mt-6 w-full bg-gray-200 text-gray-800 py-2 rounded hover:bg-gray-300 text-base"
        onClick={handleFeedback}
      >
        Feedback / Report Issue
      </button>
    </div>
  );
}