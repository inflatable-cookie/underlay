type UnderlayComponent = import("svelte").Component<Record<string, unknown>>;

declare const Badge: UnderlayComponent;
declare const Breadcrumbs: UnderlayComponent;
declare const Card: UnderlayComponent;
declare const Pill: UnderlayComponent;
declare const TextArea: UnderlayComponent;
declare const MarkdownEditor: UnderlayComponent;
declare const Form: UnderlayComponent;
declare const ActionArea: UnderlayComponent;
declare const FormActions: UnderlayComponent;
declare const Switch: UnderlayComponent;
declare const ListGrid: UnderlayComponent;
declare const ListCard: UnderlayComponent;
declare const VideoPlayer: UnderlayComponent;
declare const Select: UnderlayComponent;
declare const ConfirmAction: UnderlayComponent;
declare const ToastHost: UnderlayComponent;
declare const DropdownMenu: UnderlayComponent;
declare const Tooltip: UnderlayComponent;
declare const TimeAgo: UnderlayComponent;
declare const DateRange: UnderlayComponent;
declare const Popover: UnderlayComponent;
declare const Skeleton: UnderlayComponent;
declare const DataTable: UnderlayComponent;
declare const DetailsCard: UnderlayComponent;
declare const DetailsItem: UnderlayComponent;
declare const DetailsSection: UnderlayComponent;
declare const ContentCard: UnderlayComponent;
declare const TabsRoot: UnderlayComponent;
declare const TabsList: UnderlayComponent;
declare const TabsTrigger: UnderlayComponent;
declare const TabsContent: UnderlayComponent;
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

declare const LoginForm: UnderlayComponent;
declare const RegisterForm: UnderlayComponent;
declare const TotpSetup: UnderlayComponent;
declare const TotpInput: UnderlayComponent;
declare const PasswordRequirements: UnderlayComponent;
declare const PassKeyButton: UnderlayComponent;
declare const GoogleSignInButton: UnderlayComponent;
declare const AccountRecovery: UnderlayComponent;
declare const AuthLayout: UnderlayComponent;
declare const TwoFactorStep: UnderlayComponent;
declare const SuccessStep: UnderlayComponent;
declare const PasswordResetStep: UnderlayComponent;
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
  ConfirmAction,
  DataTable,
  DateRange,
  DetailsCard,
  DetailsItem,
  DetailsSection,
  ContentCard,
  DropdownMenu,
  Form,
  ActionArea,
  FormActions,
  LoginForm,
  RegisterForm,
  TotpSetup,
  TotpInput,
  PasswordRequirements,
  PassKeyButton,
  GoogleSignInButton,
  AccountRecovery,
  AuthLayout,
  TwoFactorStep,
  SuccessStep,
  PasswordResetStep,
  ForgotPasswordFlow,
  LoginPage,
  MarkdownEditor,
  ListCard,
  ListGrid,
  OrderBy,
  Pagination,
  Pill,
  Popover,
  Select,
  Skeleton,
  Switch,
  TabsContent,
  TabsList,
  TabsRoot,
  TabsTrigger,
  TextArea,
  TimeAgo,
  ToastHost,
  Tooltip,
  VideoPlayer,
  formatAdaptiveDateRange,
  formatDateWithOrdinal
};

export type { OrderByFieldDefinition, OrderByField, OrderByValue };
