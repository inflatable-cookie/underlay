# g05.002: compli-me and contact-patch detail-page convergence sweep

Status: complete

## Why

After Dairy `g05.001`, the next two real consumer proof sets were
`compli-me/admin` and `contact-patch/cp-admin`.

Both already had most root list ownership in decent shape.
The remaining drift was concentrated in repeated detail routes that still owned
their own outer shell:

- user detail
- system job detail
- system error detail
- system scheduled-task detail

## Consumer Upgrade Impact

This lane changes route composition in both consumers. Main risks:

- detail-page loading / error posture drift
- tab behavior drift on user and scheduled-task detail pages
- local workflow actions breaking when moved under the shared shell

The execution rule stayed the same as `g05.001`:

- move the repeated outer shell onto `EntityDetailPage`
- keep route-owned workflow behavior local
- do not widen the shared template unless repeated real callers prove a gap

## Current Inventory

### Direct migration set

`compli-me/admin`

- `/users/[userId]`
- `/system/jobs/[id]`
- `/system/errors/[id]`
- `/system/scheduled-tasks/[id]`

`contact-patch/cp-admin`

- `/users/[userId]`
- `/system/scheduled-tasks/[id]`

### Already converged before this lane

`contact-patch/cp-admin`

- `/system/jobs/[id]`

`compli-me/admin`

- the main compliments detail family already used `EntityDetailPage`

### Explicit exception

`contact-patch/cp-admin`

- `/media/[mediaId]`

That route is not another missed detail-shell migration. It is a workflow-owned
media manager with:

- preview/open behavior
- upload-new-version flow
- activate-version flow
- delete-version flow
- version-state pills
- usage management

So it stays outside `EntityDetailPage` unless another consumer proves a shared
versions-manager pattern.

## Execution Posture

Run the lane as one bounded detail-family batch:

1. migrate the repeated detail holdouts
2. recheck for remaining repeated misses
3. explicitly classify workflow exceptions instead of forcing them

## Result

The repeated detail-shell holdouts are now on `EntityDetailPage` across both
consumers.

Migrated in `compli-me/admin`:

- `/users/[userId]`
- `/system/jobs/[id]`
- `/system/errors/[id]`
- `/system/scheduled-tasks/[id]`

Migrated in `contact-patch/cp-admin`:

- `/users/[userId]`
- `/system/scheduled-tasks/[id]`

No new shared Underlay template seam was needed for this lane.

The final classification is cleaner now:

- repeated entity detail shells use `EntityDetailPage`
- route-owned workflow sections stay local inside that shell
- workflow-heavy media detail remains an explicit exception
