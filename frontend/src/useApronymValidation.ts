import { useCallback } from "react";

export interface ApronymValidation {
  validTerms: string[];
  error: string | null;
}

/**
 * Validates the input terms and form values for the Apronym generator.
 */
export function useApronymValidation() {
  return useCallback((terms: string, fragLen: number, minLen: number, maxLen: number): ApronymValidation => {
    const termsArray = terms
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t.length > 0);

    if (termsArray.length === 0) {
      return { validTerms: [], error: "Please enter at least one valid term." };
    }
    if (termsArray.length > 10) {
      return { validTerms: [], error: "Please enter no more than 10 terms." };
    }
    const uniqueTerms = Array.from(new Set(termsArray));
    if (uniqueTerms.length !== termsArray.length) {
      return { validTerms: [], error: "Terms must be unique — please remove duplicates." };
    }
    if (minLen < 1 || maxLen < 1) {
      return { validTerms: [], error: "Min Length and Max Length must both be at least 1." };
    }
    if (fragLen < 1 || fragLen > 3) {
      return { validTerms: [], error: "Fragment Length must be between 1 and 3." };
    }
    if (minLen > 10 || maxLen > 10) {
      return { validTerms: [], error: "Min Length and Max Length must not exceed 10." };
    }
    if (maxLen < minLen) {
      return { validTerms: [], error: "Max Length must be greater than or equal to Min Length." };
    }
    if (maxLen > uniqueTerms.length) {
      return { validTerms: [], error: "Max Length cannot exceed the number of terms provided." };
    }
    const validTerms = uniqueTerms.filter((t) => /^[a-zA-Z]+$/.test(t));
    if (validTerms.length !== uniqueTerms.length) {
      return { validTerms: [], error: "Terms must only contain letters (A-Z)." };
    }
    const hasVowelStart = validTerms.some((t) => /^[aeiouAEIOU]/.test(t));
    if (!hasVowelStart) {
      return { validTerms: [], error: "At least one term must start with a vowel (A, E, I, O, U)." };
    }
    // Fragment length validation: must be <= length of smallest term
    const minTermLength = uniqueTerms.reduce((min, term) => Math.min(min, term.length), Infinity);
    if (fragLen > minTermLength) {
      return { validTerms: [], error: `Fragment Length (${fragLen}) must not be greater than the length of the smallest term (${minTermLength}).` };
    }
    return { validTerms, error: null };
  }, []);
}
