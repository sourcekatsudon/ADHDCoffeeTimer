import { localizeError, type Language } from './i18n';
import { visibleActivityIssue, type Settings, type Snapshot } from './model';

export function ActivityIssueAlert({
  status,
  activityIssue,
  autoDetect,
  language,
}: {
  status: Snapshot['status'];
  activityIssue?: string | null;
  autoDetect: Settings['autoDetect'];
  language: Language;
}) {
  const issue = visibleActivityIssue({ status, activityIssue, settings: { autoDetect } });
  if (!issue) return null;
  return (
    <p className="form-error" role="alert">
      {localizeError(issue, language)}
    </p>
  );
}
