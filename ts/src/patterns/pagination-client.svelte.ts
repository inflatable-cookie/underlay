import type { PaginationController } from "./pagination-types";
import { DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE } from "./pagination-types";

export interface ClientPaginationOptions {
  pageSize?: number;
  initialPage?: number;
  persistKey?: string;
}

export function createClientPagination<T>(
  getAllItems: () => T[],
  options: ClientPaginationOptions = {},
): PaginationController<T> {
  const getInitialPageSize = (): number => {
    if (options.persistKey && typeof localStorage !== "undefined") {
      const stored = localStorage.getItem(options.persistKey);
      if (stored) {
        const parsed = parseInt(stored, 10);
        if (!isNaN(parsed) && parsed > 0 && parsed <= MAX_PAGE_SIZE) {
          return parsed;
        }
      }
    }
    return options.pageSize ?? DEFAULT_PAGE_SIZE;
  };

  let currentPage = $state(options.initialPage ?? 1);
  let pageSize = $state(Math.min(getInitialPageSize(), MAX_PAGE_SIZE));

  const allItems = $derived(getAllItems());
  const total = $derived(allItems.length);
  const totalPages = $derived(Math.max(1, Math.ceil(total / pageSize)));
  const validPage = $derived(Math.min(Math.max(1, currentPage), totalPages));

  const items = $derived.by(() => {
    const start = (validPage - 1) * pageSize;
    const end = start + pageSize;
    return allItems.slice(start, end);
  });

  const hasNextPage = $derived(validPage < totalPages);
  const hasPrevPage = $derived(validPage > 1);
  const showingFrom = $derived(total > 0 ? (validPage - 1) * pageSize + 1 : 0);
  const showingTo = $derived(
    showingFrom > 0 ? Math.min(showingFrom + pageSize - 1, total) : 0,
  );

  const nextPage = () => {
    if (hasNextPage) {
      currentPage = validPage + 1;
    }
  };

  const prevPage = () => {
    if (hasPrevPage) {
      currentPage = validPage - 1;
    }
  };

  const goToPage = (page: number) => {
    currentPage = Math.min(Math.max(1, page), totalPages);
  };

  const setPageSize = (size: number) => {
    const newSize = Math.min(Math.max(1, size), MAX_PAGE_SIZE);
    if (newSize === pageSize) return;

    const firstItemIndex = (validPage - 1) * pageSize;
    pageSize = newSize;
    currentPage = Math.floor(firstItemIndex / newSize) + 1;

    if (options.persistKey && typeof localStorage !== "undefined") {
      localStorage.setItem(options.persistKey, String(newSize));
    }
  };

  const refresh = async () => {};

  const reset = async () => {
    currentPage = 1;
  };

  return {
    get items() {
      return items;
    },
    get currentPage() {
      return validPage;
    },
    get pageSize() {
      return pageSize;
    },
    get hasNextPage() {
      return hasNextPage;
    },
    get hasPrevPage() {
      return hasPrevPage;
    },
    get total() {
      return total;
    },
    get loading() {
      return false;
    },
    get error() {
      return null;
    },
    get showingFrom() {
      return showingFrom;
    },
    get showingTo() {
      return showingTo;
    },
    get totalPages() {
      return totalPages;
    },
    nextPage,
    prevPage,
    goToPage,
    setPageSize,
    refresh,
    reset,
  };
}
