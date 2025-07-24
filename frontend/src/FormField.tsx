import React, { useState, useEffect } from "react";

interface FormFieldProps {
  label: string;
  type: string;
  value: string | number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  min?: number;
  max?: number;
  inputMode?: "search" | "text" | "email" | "tel" | "url" | "numeric" | "none" | "decimal";
  autoComplete?: string;
}

/**
 * Reusable form field for ApronymForm.
 */
export function FormField({ label, type, value, onChange, min, max, inputMode, autoComplete }: FormFieldProps) {
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    // Detect if user is on mobile device
    const checkMobile = () => {
      const userAgent = navigator.userAgent.toLowerCase();
      const isMobileDevice = /android|webos|iphone|ipad|ipod|blackberry|iemobile|opera mini/.test(userAgent);
      const isTouchDevice = 'ontouchstart' in window || navigator.maxTouchPoints > 0;
      setIsMobile(isMobileDevice || isTouchDevice);
    };
    
    checkMobile();
    // Also check on window resize in case device orientation changes
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  // For numeric fields on mobile, use text input with pattern and inputMode
  // For desktop, keep native number input with spinners
  const shouldUseMobileInput = type === "number" && isMobile;
  const inputType = shouldUseMobileInput ? "text" : type;
  const pattern = shouldUseMobileInput ? "[0-9]*" : undefined;
  const effectiveInputMode = type === "number" ? "numeric" : inputMode;

  // Handle numeric input validation for mobile text-based numeric fields
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (shouldUseMobileInput) {
      const newValue = e.target.value;
      // Allow empty string for clearing
      if (newValue === "") {
        onChange(e);
        return;
      }
      // Only allow numeric characters
      if (!/^\d+$/.test(newValue)) {
        return; // Don't update if non-numeric
      }
      // Validate against min/max if provided
      const numValue = parseInt(newValue, 10);
      if (min !== undefined && numValue < min) {
        return;
      }
      if (max !== undefined && numValue > max) {
        return;
      }
    }
    onChange(e);
  };

  return (
    <div>
      <label className="block font-semibold text-base sm:text-lg">{label}</label>
      <input
        type={inputType}
        value={value}
        onChange={handleChange}
        className="w-full border rounded p-3 text-base sm:text-lg"
        min={!shouldUseMobileInput ? min : undefined}
        max={!shouldUseMobileInput ? max : undefined}
        pattern={pattern}
        inputMode={effectiveInputMode}
        autoComplete={autoComplete}
        aria-label={label}
        placeholder={shouldUseMobileInput && min !== undefined ? `Min: ${min}` : undefined}
      />
    </div>
  );
}
