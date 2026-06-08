type ContextLike = {
  label: string;
  href: string;
  targetHref?: string;
};

type PathMatcher = (targetHref: string) => boolean;

export interface BackInfoLike {
  label: string;
  href: string;
  isContextual?: boolean;
}

function isContextValid(
  context: ContextLike,
  matchesCurrentPath: PathMatcher,
): boolean {
  return !context.targetHref || matchesCurrentPath(context.targetHref);
}

function contextualBackLabel(label: string): string {
  const trimmed = label.trim();
  if (!trimmed) return "Back";
  return trimmed;
}

export function resolveBackButtonInfo(
  context: ContextLike | null,
  fallbackLabel: string,
  fallbackHref: string,
  matchesCurrentPath: PathMatcher,
): BackInfoLike {
  if (context?.targetHref && !matchesCurrentPath(context.targetHref)) {
    return {
      label: fallbackLabel,
      href: fallbackHref,
      isContextual: false,
    };
  }

  if (context) {
    return {
      label: contextualBackLabel(context.label),
      href: context.href,
      isContextual: true,
    };
  }

  return {
    label: fallbackLabel,
    href: fallbackHref,
    isContextual: false,
  };
}

export function consumeBackNavigation(
  context: ContextLike | null,
  fallbackLabel: string,
  fallbackHref: string,
  matchesCurrentPath: PathMatcher,
): { backInfo: BackInfoLike; returnTo: string } {
  if (context && isContextValid(context, matchesCurrentPath)) {
    return {
      backInfo: {
        label: contextualBackLabel(context.label),
        href: context.href,
        isContextual: true,
      },
      returnTo: context.href,
    };
  }

  return {
    backInfo: {
      label: fallbackLabel,
      href: fallbackHref,
      isContextual: false,
    },
    returnTo: fallbackHref,
  };
}

export function computeResolvedBackInfo(
  backInfo: BackInfoLike,
  fallback?: { href: string; label: string },
): BackInfoLike {
  if (backInfo.isContextual) {
    return backInfo;
  }

  if (fallback) {
    return {
      href: fallback.href,
      label: fallback.label,
      isContextual: false,
    };
  }

  return backInfo;
}
