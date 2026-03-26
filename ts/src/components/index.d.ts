type UnderlayComponent = import("svelte").Component<Record<string, unknown>>;

declare const Badge: UnderlayComponent;
declare const Breadcrumbs: UnderlayComponent;
declare const Card: UnderlayComponent;
declare const Pill: UnderlayComponent;
declare const TextArea: UnderlayComponent;
declare const MarkdownEditor: UnderlayComponent;
declare const FormActions: UnderlayComponent;
declare const Switch: UnderlayComponent;
declare const ListGrid: UnderlayComponent;
declare const ListCard: UnderlayComponent;
declare const VideoPlayer: UnderlayComponent;
declare const Select: UnderlayComponent;
declare const ToastHost: UnderlayComponent;
declare const DropdownMenu: UnderlayComponent;
declare const TimeAgo: UnderlayComponent;
declare const DateRange: UnderlayComponent;
declare const Skeleton: UnderlayComponent;
declare const DataTable: UnderlayComponent;
declare const DetailsCard: UnderlayComponent;
declare const DetailsItem: UnderlayComponent;
declare const DetailsSection: UnderlayComponent;
declare const Pagination: UnderlayComponent;
declare const OrderBy: UnderlayComponent;

interface OrderByFieldDefinition {
  key: string;
  label: string;
  defaultDirection?: "asc" | "desc";
}

interface OrderByField {
  key: string;
  direction: "asc" | "desc";
}

type OrderByValue = OrderByField[];

declare const TotpInput: UnderlayComponent;
declare const PasswordRequirements: UnderlayComponent;
declare const AuthLayout: UnderlayComponent;
declare const ForgotPasswordFlow: UnderlayComponent;
declare const LoginPage: UnderlayComponent;
declare function formatAdaptiveDateRange(
  startInput: string | Date | null | undefined,
  endInput: string | Date | null | undefined,
  options?: { locale?: string; style?: "adaptive" | "full" }
): string | null;
declare function formatDateWithOrdinal(
  input: string | Date | null | undefined,
  locale?: string
): string | null;

export {
  Badge,
  Breadcrumbs,
  Card,
  DataTable,
  DateRange,
  DetailsCard,
  DetailsItem,
  DetailsSection,
  DropdownMenu,
  FormActions,
  TotpInput,
  PasswordRequirements,
  AuthLayout,
  ForgotPasswordFlow,
  LoginPage,
  MarkdownEditor,
  ListCard,
  ListGrid,
  OrderBy,
  Pagination,
  Pill,
  Select,
  Skeleton,
  Switch,
  TextArea,
  TimeAgo,
  ToastHost,
  VideoPlayer,
  formatAdaptiveDateRange,
  formatDateWithOrdinal
};

export type { OrderByFieldDefinition, OrderByField, OrderByValue };
