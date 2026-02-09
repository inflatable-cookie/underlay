import type { NavigationContext } from "./navigation";

/**
 * Compute the next navigation context stack after pushing a context.
 *
 * Applies:
 * - href deduplication
 * - list->list top collapse
 * - max depth trimming
 */
export function buildPushedContextStack(
  stack: NavigationContext[],
  context: NavigationContext,
  maxDepth: number
): NavigationContext[] {
  let nextStack = stack.filter((item) => item.href !== context.href);

  const shouldCollapseSameType = context.type === "list";

  if (
    shouldCollapseSameType &&
    nextStack.length > 0 &&
    nextStack[nextStack.length - 1].type === context.type
  ) {
    nextStack[nextStack.length - 1] = context;
  } else {
    nextStack.push(context);
  }

  if (nextStack.length > maxDepth) {
    nextStack = nextStack.slice(-maxDepth);
  }

  return nextStack;
}
