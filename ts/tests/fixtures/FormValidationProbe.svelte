<script lang="ts">
  import { getContext } from "svelte";

  type FormValidationContext = {
    registerField: (
      id: string,
      required: boolean,
      hasValue: boolean,
      validationStatus: string,
      isValidationValid: boolean
    ) => void;
    unregisterField: (id: string) => void;
    updateField: (
      id: string,
      hasValue: boolean,
      validationStatus?: string,
      isValidationValid?: boolean
    ) => void;
  };

  const ctx = getContext<FormValidationContext>("formValidation");

  function registerInvalid() {
    ctx.registerField("field-a", true, false, "idle", true);
  }

  function registerValid() {
    ctx.registerField("field-a", true, true, "idle", true);
  }

  function setValidating() {
    ctx.updateField("field-a", true, "validating", false);
  }

  function setValid() {
    ctx.updateField("field-a", true, "valid", true);
  }

  function unregister() {
    ctx.unregisterField("field-a");
  }
</script>

<button type="button" data-testid="register-invalid" onclick={registerInvalid}>Register Invalid</button>
<button type="button" data-testid="register-valid" onclick={registerValid}>Register Valid</button>
<button type="button" data-testid="set-validating" onclick={setValidating}>Set Validating</button>
<button type="button" data-testid="set-valid" onclick={setValid}>Set Valid</button>
<button type="button" data-testid="unregister" onclick={unregister}>Unregister</button>
