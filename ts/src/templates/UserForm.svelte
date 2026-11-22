<script lang="ts">
  import {
    Button,
    Field,
    FieldSet,
    FormActions,
    SplitButton,
    Select,
    Switch,
    TextInput
  } from "@inflatable-cookie/poodle-svelte";
  import { navigateOnCancel } from "../client/navigation";
  import { untrack } from "svelte";
  import {
    DEFAULT_USER_ROLE_OPTIONS,
    DEFAULT_USER_STATUS_OPTIONS
  } from "./users-list.types";

  type UserFormMode = "create" | "edit";

  interface UserFormValues {
    email?: string;
    displayName?: string | null;
    role?: string;
    status?: string;
    sendPasswordReset?: boolean;
  }

  interface Props {
    mode?: UserFormMode;
    values?: UserFormValues;
    intent?: "save" | "save-close";
    errors?: Record<string, string> | null;
    cancelHref?: string;
    returnTo?: string;
    roleOptions?: { value: string; label: string }[];
    statusOptions?: { value: string; label: string }[];
    emailDisabledInEdit?: boolean;
  }

  let {
    mode = "edit",
    values = {},
    intent = $bindable("save-close"),
    errors = null,
    cancelHref = undefined,
    returnTo = undefined,
    roleOptions = DEFAULT_USER_ROLE_OPTIONS.filter((o) => o.value !== "All"),
    statusOptions = DEFAULT_USER_STATUS_OPTIONS.filter((o) => o.value !== "All"),
    emailDisabledInEdit = false
  }: Props = $props();

  const uid = $props.id();
  const emailFieldId = `${uid}-user-email`;
  const displayNameFieldId = `${uid}-user-display-name`;
  const roleFieldId = `${uid}-user-role`;
  const statusFieldId = `${uid}-user-status`;
  const sendPasswordResetFieldId = `${uid}-user-send-password-reset`;

  let emailValue = $state(untrack(() => values.email ?? ""));
  let displayNameValue = $state(untrack(() => values.displayName ?? ""));
  let roleValue = $state(untrack(() => values.role ?? "user"));
  let statusValue = $state(untrack(() => values.status ?? "active"));
  let sendPasswordResetValue = $state(untrack(() => values.sendPasswordReset ?? true));

  // Track last synced prop values in plain variables so async edit data updates
  // populate the fields without clobbering user edits on unrelated parent re-renders.
  let lastPropEmail = untrack(() => values.email ?? "");
  let lastPropDisplayName = untrack(() => values.displayName ?? "");
  let lastPropRole = untrack(() => values.role ?? "user");
  let lastPropStatus = untrack(() => values.status ?? "active");
  let lastPropSendPasswordReset = untrack(() => values.sendPasswordReset ?? true);

  $effect(() => {
    const nextEmail = values.email ?? "";
    const nextDisplayName = values.displayName ?? "";
    const nextRole = values.role ?? "user";
    const nextStatus = values.status ?? "active";
    const nextSendPasswordReset = values.sendPasswordReset ?? true;

    if (nextEmail !== lastPropEmail) {
      lastPropEmail = nextEmail;
      emailValue = nextEmail;
    }
    if (nextDisplayName !== lastPropDisplayName) {
      lastPropDisplayName = nextDisplayName;
      displayNameValue = nextDisplayName;
    }
    if (nextRole !== lastPropRole) {
      lastPropRole = nextRole;
      roleValue = nextRole;
    }
    if (nextStatus !== lastPropStatus) {
      lastPropStatus = nextStatus;
      statusValue = nextStatus;
    }
    if (nextSendPasswordReset !== lastPropSendPasswordReset) {
      lastPropSendPasswordReset = nextSendPasswordReset;
      sendPasswordResetValue = nextSendPasswordReset;
    }
  });

  const editIntentItems = [
    { value: "save", label: "Save changes" },
    { value: "save-close", label: "Save & close" }
  ];

  let actionBarElement = $state<HTMLDivElement | null>(null);

  const isFormValid = $derived.by(() => {
    const email = emailValue.trim();
    return Boolean(email && email.includes("@") && roleValue && statusValue);
  });

  $effect(() => {
    if (mode === "create") {
      intent = "save-close";
    }
  });

  function handleCancel() {
    navigateOnCancel(cancelHref);
  }

  function validationState(error?: string | null) {
    return error ? "invalid" : "none";
  }

  function submitWithIntent(nextIntent: "save" | "save-close") {
    intent = nextIntent;
    actionBarElement?.closest("form")?.requestSubmit();
  }
</script>

<FieldSet legend="User" columns={2}>
  <Field
    id={emailFieldId}
    label="Email"
    error={errors?.email ?? null}
    validationState={validationState(errors?.email)}
    required
  >
    {#snippet control({ describedBy, validationState })}
      <TextInput
        id={emailFieldId}
        name="email"
        type="email"
        inputMode="email"
        value={emailValue}
        describedBy={describedBy}
        validationState={validationState}
        placeholder="name@example.com"
        maxLength={320}
        disabled={mode === "edit" && emailDisabledInEdit}
        onValueChange={(nextValue) => { emailValue = nextValue; }}
      />
    {/snippet}
  </Field>

  <Field
    id={displayNameFieldId}
    label="Display Name"
    error={errors?.displayName ?? null}
    validationState={validationState(errors?.displayName)}
  >
    {#snippet control({ describedBy, validationState })}
      <TextInput
        id={displayNameFieldId}
        name="displayName"
        value={displayNameValue}
        describedBy={describedBy}
        validationState={validationState}
        placeholder="Optional"
        maxLength={100}
        onValueChange={(nextValue) => { displayNameValue = nextValue; }}
      />
    {/snippet}
  </Field>
</FieldSet>

<FieldSet legend="Access" columns={2}>
  <Field
    id={roleFieldId}
    label="Role"
    error={errors?.role ?? null}
    validationState={validationState(errors?.role)}
    required
  >
    {#snippet control({ describedBy })}
      <Select
        id={roleFieldId}
        name="role"
        value={roleValue}
        describedBy={describedBy}
        options={roleOptions}
        placeholder="Select role"
        onValueChange={(value) => { roleValue = value; }}
      />
    {/snippet}
  </Field>

  <Field
    id={statusFieldId}
    label="Status"
    error={errors?.status ?? null}
    validationState={validationState(errors?.status)}
    required
  >
    {#snippet control({ describedBy })}
      <Select
        id={statusFieldId}
        name="status"
        value={statusValue}
        describedBy={describedBy}
        options={statusOptions}
        placeholder="Select status"
        onValueChange={(value) => { statusValue = value; }}
      />
    {/snippet}
  </Field>
</FieldSet>

{#if mode === "create"}
  <FieldSet legend="Onboarding">
    <Field
      id={sendPasswordResetFieldId}
      label="Send password reset email"
      hint="Sends a password reset email so the user can set an initial password."
      error={errors?.sendPasswordReset ?? null}
      validationState={validationState(errors?.sendPasswordReset)}
    >
      {#snippet control({ describedBy })}
        <input
          type="hidden"
          name="sendPasswordReset"
          value={sendPasswordResetValue ? "true" : "false"}
        />
        <div class="underlay-user-form__switch-row">
          <span class="underlay-user-form__switch-label">No</span>
          <Switch
            id={sendPasswordResetFieldId}
            name="sendPasswordReset-switch"
            checked={sendPasswordResetValue}
            describedBy={describedBy}
            ariaLabel="Send password reset email"
            onCheckedChange={(checked) => { sendPasswordResetValue = checked; }}
          />
          <span class="underlay-user-form__switch-label">Yes</span>
        </div>
      {/snippet}
    </Field>
  </FieldSet>
{/if}

<FormActions align="end" showTopBorder>
  <div class="underlay-user-form__actions" bind:this={actionBarElement}>
    <input type="hidden" name="intent" value={intent} />

    {#if returnTo}
      <input type="hidden" name="returnTo" value={returnTo} />
    {/if}

    <Button type="button" variant="ghost" onClick={handleCancel}>
      Cancel
    </Button>

    {#if mode === "create"}
      <Button type="submit" variant="primary" disabled={!isFormValid}>
        Create user
      </Button>
    {:else}
      <SplitButton
        variant="primary"
        items={editIntentItems}
        disabled={!isFormValid}
        onClick={() => submitWithIntent(intent)}
        onAction={(value) => submitWithIntent(value as "save" | "save-close")}
      >
        {intent === "save" ? "Save changes" : "Save & close"}
      </SplitButton>
    {/if}
  </div>
</FormActions>

<style>
  .underlay-user-form__actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .underlay-user-form__switch-row {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  .underlay-user-form__switch-label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    line-height: var(--poodle-typography-label-lineHeight);
  }
</style>
