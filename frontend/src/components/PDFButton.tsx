import { useState } from "react";

export default function PDFButton() {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [_error, setError] = useState<string | null>(null);
  const API_URL = import.meta.env.VITE_API_URL || '/api';

  // Fetch PDF from Axum server
  const fetchPdf = async (): Promise<void> => {
    setLoading(true);
    setError(null);

    try {
      const response: Response = await fetch(`${API_URL}/pdf/example`, {
        method: 'GET',
        headers: {
          'Accept': 'application/pdf',
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      // Get the PDF as a blob (binary data)
      const blob: Blob = await response.blob();

      // Create a URL for the blob
      const url: string = URL.createObjectURL(blob);
      setPdfUrl(url);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <button onClick={fetchPdf} disabled={loading}>
        {loading ? "Loading..." : "Generate PDF"}
      </button>
      {pdfUrl && (
        <div>
          <iframe
            style={{ width: "800px", height: "1300px" }}
            src={pdfUrl}
            title="PDF Viewer"
          />
        </div>
      )}

    </>
  )
}
