import React from "react";

interface SliderFieldProps {
  label: string;
  value: number;
  min: number;
  max: number;
  step?: number;
  onChange: (value: number) => void;
  disabled?: boolean;
}

export const SliderField: React.FC<SliderFieldProps> = ({
  label,
  value,
  min,
  max,
  step = 1,
  onChange,
  disabled = false,
}) => {
  // Clamp value to min/max
  const safeValue = Math.max(min, Math.min(max, value));

  return (
    <div className="flex flex-col gap-1">
      <label className="font-medium text-sm mb-1">{label}</label>
      <div className="flex items-center gap-3">
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={safeValue}
          onChange={e => onChange(Number(e.target.value))}
          className="flex-1 accent-blue-500"
          disabled={disabled}
        />
        <input
          type="number"
          min={min}
          max={max}
          step={step}
          value={safeValue}
          onChange={e => {
            const val = e.target.value === "" ? min : Number(e.target.value);
            onChange(val);
          }}
          className="w-16 px-2 py-1 border rounded text-center"
          disabled={disabled}
        />
      </div>
      <div className="text-xs text-gray-500 mt-1">
        Range: {min} – {max}
      </div>
    </div>
  );
};
