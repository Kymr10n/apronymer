import { useEffect, useState } from "react";
import ApronymForm from './ApronymForm';

function App() {
  const [gitVersion, setGitVersion] = useState<string>("");

  useEffect(() => {
    fetch("/gitversion.txt")
      .then((res) => res.text())
      .then((txt) => setGitVersion(txt.trim()));
  }, []);

  return (
    <div className="max-w-md mx-auto px-2 sm:px-4 py-4">
      <h1 className="text-2xl font-bold text-left mb-2">
        Apronymers
        {gitVersion && (
          <span className="ml-2 text-xs text-gray-500 align-middle">({gitVersion})</span>
        )}
      </h1>
      <div>
        <ApronymForm />
      </div>
    </div>
  );
}

export default App;