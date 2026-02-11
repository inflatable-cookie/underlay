declare const Badge: any;
declare const Breadcrumbs: any;
declare const Button: any;
declare const TextButton: any;
declare const Card: any;
declare const Field: any;
declare const FieldHint: any;
declare const FieldSet: any;
declare const FieldSetGrid: any;
declare const Pill: any;
declare const TextInput: any;
declare const TextArea: any;
declare const MarkdownEditor: any;
declare const Form: any;
declare const FormActions: any;
declare const FormError: any;
declare const Switch: any;
declare const ListGrid: any;
declare const ListCard: any;
declare const SplitButton: any;
declare const SaveSplitButton: any;
declare const VideoPlayer: any;
declare const Select: any;
declare const Dialog: any;
declare const AlertDialog: any;
declare const ConfirmAction: any;
declare const ToastHost: any;
declare const DropdownMenu: any;
declare const IconButton: any;
declare const Tooltip: any;
declare const TimeAgo: any;
declare const DateRange: any;
declare const Popover: any;
declare const Skeleton: any;
declare const DataTable: any;
declare const DetailsCard: any;
declare const DetailsItem: any;
declare const DetailsSection: any;
declare const ContentCard: any;
declare const FileUpload: any;
declare const TabsRoot: any;
declare const TabsList: any;
declare const TabsTrigger: any;
declare const TabsContent: any;
declare const Pagination: any;
declare const OrderBy: any;

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

declare const LoginForm: any;
declare const RegisterForm: any;
declare const TotpSetup: any;
declare const TotpInput: any;
declare const PasswordRequirements: any;
declare const PassKeyButton: any;
declare const GoogleSignInButton: any;
declare const SessionList: any;
declare const SecuritySettings: any;
declare const AccountRecovery: any;
declare const AuthLayout: any;
declare const TwoFactorStep: any;
declare const SuccessStep: any;
declare const PasswordResetStep: any;
declare const ForgotPasswordFlow: any;
declare const LoginPage: any;
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
  AlertDialog,
  Badge,
  Breadcrumbs,
  Button,
  Card,
  ConfirmAction,
  DataTable,
  DateRange,
  DetailsCard,
  DetailsItem,
  DetailsSection,
  ContentCard,
  Dialog,
  DropdownMenu,
  Field,
  FieldHint,
  FieldSet,
  FieldSetGrid,
  FileUpload,
  Form,
  FormActions,
  FormError,
  LoginForm,
  RegisterForm,
  TotpSetup,
  TotpInput,
  PasswordRequirements,
  PassKeyButton,
  GoogleSignInButton,
  SessionList,
  SecuritySettings,
  AccountRecovery,
  AuthLayout,
  TwoFactorStep,
  SuccessStep,
  PasswordResetStep,
  ForgotPasswordFlow,
  LoginPage,
  MarkdownEditor,
  IconButton,
  ListCard,
  ListGrid,
  OrderBy,
  Pagination,
  Pill,
  Popover,
  SaveSplitButton,
  Select,
  Skeleton,
  SplitButton,
  Switch,
  TabsContent,
  TabsList,
  TabsRoot,
  TabsTrigger,
  TextArea,
  TextButton,
  TextInput,
  TimeAgo,
  ToastHost,
  Tooltip,
  VideoPlayer,
  formatAdaptiveDateRange,
  formatDateWithOrdinal
};

export type { OrderByFieldDefinition, OrderByField, OrderByValue };
