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
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">
        Apronymer
        {gitVersion && (
          <span className="ml-2 text-xs text-gray-500 align-middle">({gitVersion})</span>
        )}
      </h1>
      <ApronymForm />
    </div>
  );
}

export default App;