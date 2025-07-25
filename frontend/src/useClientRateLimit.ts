import { useState, useCallback, useRef } from 'react';

interface RateLimitConfig {
  maxRequests: number;
  windowMs: number;
  cooldownMs?: number;
}

interface RateLimitState {
  canSubmit: boolean;
  requestsRemaining: number;
  timeUntilReset: number;
  lastBlockedReason?: string;
}

/**
 * Client-side rate limiting hook to prevent rapid form submissions
 * Works alongside backend rate limiting for better UX
 */
export function useClientRateLimit(config: RateLimitConfig = {
  maxRequests: 5,        // 5 requests
  windowMs: 60 * 1000,   // per minute
  cooldownMs: 2 * 1000   // 2 second cooldown between requests
}) {
  const [state, setState] = useState<RateLimitState>({
    canSubmit: true,
    requestsRemaining: config.maxRequests,
    timeUntilReset: 0
  });

  const requestTimestamps = useRef<number[]>([]);
  const lastRequestTime = useRef<number>(0);

  const updateState = useCallback(() => {
    const now = Date.now();
    const windowStart = now - config.windowMs;

    // Clean old timestamps outside the window
    requestTimestamps.current = requestTimestamps.current.filter(
      timestamp => timestamp > windowStart
    );

    const requestsInWindow = requestTimestamps.current.length;
    const requestsRemaining = Math.max(0, config.maxRequests - requestsInWindow);
    
    // Check cooldown period
    const timeSinceLastRequest = now - lastRequestTime.current;
    const inCooldown = config.cooldownMs && timeSinceLastRequest < config.cooldownMs;
    
    // Calculate time until reset
    const oldestRequest = requestTimestamps.current[0];
    const timeUntilReset = oldestRequest ? Math.max(0, (oldestRequest + config.windowMs) - now) : 0;

    // Determine if can submit and why not
    let canSubmit = true;
    let lastBlockedReason: string | undefined;

    if (inCooldown) {
      canSubmit = false;
      lastBlockedReason = `Please wait ${Math.ceil((config.cooldownMs! - timeSinceLastRequest) / 1000)} seconds before submitting again`;
    } else if (requestsRemaining === 0) {
      canSubmit = false;
      const resetMinutes = Math.ceil(timeUntilReset / (60 * 1000));
      lastBlockedReason = `Rate limit reached. Try again in ${resetMinutes} minute${resetMinutes !== 1 ? 's' : ''}`;
    }

    setState({
      canSubmit,
      requestsRemaining,
      timeUntilReset,
      lastBlockedReason
    });
  }, [config.maxRequests, config.windowMs, config.cooldownMs]);

  const attemptRequest = useCallback((): boolean => {
    const now = Date.now();
    
    updateState();
    
    if (!state.canSubmit) {
      return false;
    }

    // Record this request
    requestTimestamps.current.push(now);
    lastRequestTime.current = now;
    
    // Update state immediately after recording request
    setTimeout(updateState, 0);
    
    return true;
  }, [state.canSubmit, updateState]);

  const reset = useCallback(() => {
    requestTimestamps.current = [];
    lastRequestTime.current = 0;
    setState({
      canSubmit: true,
      requestsRemaining: config.maxRequests,
      timeUntilReset: 0
    });
  }, [config.maxRequests]);

  // Auto-update state periodically
  const startAutoUpdate = useCallback(() => {
    const interval = setInterval(updateState, 1000);
    return () => clearInterval(interval);
  }, [updateState]);

  return {
    ...state,
    attemptRequest,
    reset,
    startAutoUpdate,
    config
  };
}
