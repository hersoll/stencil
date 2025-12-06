import { useState } from 'react';
import './App.css'

function App() {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [_error, setError] = useState<string | null>(null);
  const API_URL = import.meta.env.VITE_API_URL || '/api';

  // Fetch PDF from Axum server
  const fetchPdf = async (): Promise<void> => {
    setLoading(true);
    setError(null);

    try {
      // Replace with your actual Axum server endpoint
      const response: Response = await fetch(`${API_URL}/pdf`, {
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

  const downloadPdf = async (): Promise<void> => {
    setLoading(true);
    setError(null);

    try {
      const response: Response = await fetch(`${API_URL}/pdf`);

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const blob: Blob = await response.blob();

      // Create download link
      const url: string = URL.createObjectURL(blob);
      const a: HTMLAnchorElement = document.createElement('a');
      a.href = url;
      a.download = 'document.pdf';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <h1>Stencil</h1>
      <button onClick={fetchPdf} disabled={loading} className='bg-red'>
        {loading ? "Loading..." : "Generate PDF"}
      </button>
      <button
        onClick={downloadPdf}
        disabled={loading}
      >
        {loading ? 'Downloading...' : 'Download PDF'}
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

export default App
