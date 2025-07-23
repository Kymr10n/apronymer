import { FiDownload, FiCopy } from "react-icons/fi";

export interface Apronym {
  text: string;
  terms: string[];
}

interface ResultsListProps {
  results: Apronym[];
  loading: boolean;
}

/**
 * Displays the list of generated apronynms and provides copy/download actions.
 */
export function ResultsList({ results, loading }: ResultsListProps) {
  const handleCopy = () => {
    if (results.length === 0) return;
    const text = results.map(r => `${r.text}: ${r.terms.join(", ")}`).join("\n");
    navigator.clipboard.writeText(text);
  };
  const handleDownload = () => {
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
  };
  return (
    <div className="mt-6">
      <div className="flex items-center gap-2 mb-2">
        <h2 className="text-xl font-bold">Results:</h2>
        <button type="button" title="Copy results" className="p-2 rounded hover:bg-gray-200" onClick={handleCopy} disabled={results.length === 0}>
          <FiCopy size={20} />
        </button>
        <button type="button" title="Download results" className="p-2 rounded hover:bg-gray-200" onClick={handleDownload} disabled={results.length === 0}>
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
  );
}
