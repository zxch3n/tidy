import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';

const rootElement: HTMLElement | null = document.getElementById('root');
if (!rootElement) {
  throw new Error(
    "The HTML root element doesn't exist. Please report this error!",
  );
}

const root = createRoot(rootElement);

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
