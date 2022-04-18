import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [markdown, setMarkdown] = useState("# hello");
  const [html, setHtml] = useState("");

  useEffect(() => {
    invoke("render_markdown", { input: markdown }).then((output) => {
      setHtml(output);
    });
  }, [markdown]);

  return (
    <div className="relative bg-white flex gap-6 min-h-screen">
      <textarea
        className="flex-1 p-6"
        defaultValue={markdown}
        onChange={(e) => {
          setMarkdown(e.target.value);
        }}
      />

      <div
        className="prose prose-indigo text-gray-500 bg-slate-100 flex-1 min-h-full p-6"
        dangerouslySetInnerHTML={{ __html: html }}
      ></div>
    </div>
  );
}

export default App;
