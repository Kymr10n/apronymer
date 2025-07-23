import React from "react";

interface FormFieldProps {
  label: string;
  type: string;
  value: string | number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  min?: number;
  max?: number;
  inputMode?: string;
  autoComplete?: string;
}

/**
 * Reusable form field for ApronymForm.
 */
export function FormField({ label, type, value, onChange, min, max, inputMode, autoComplete }: FormFieldProps) {
  return (
    <div>
      <label className="block font-semibold text-base sm:text-lg">{label}</label>
      <input
        type={type}
        value={value}
        onChange={onChange}
        className="w-full border rounded p-3 text-base sm:text-lg"
        min={min}
        max={max}
        inputMode={inputMode as any}
        autoComplete={autoComplete}
        aria-label={label}
      />
    </div>
  );
}
