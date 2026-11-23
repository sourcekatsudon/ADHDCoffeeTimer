import { describe, expect, it } from 'vitest';
import { renderToStaticMarkup } from 'react-dom/server';
import { ActivityIssueAlert } from './activity-issue';

describe('ActivityIssueAlert', () => {
  it('renders the localized reason when autoDetect is enabled even while paused (e.g. restored checkpoint)', () => {
    const markup = renderToStaticMarkup(
      <ActivityIssueAlert
        status="paused"
        activityIssue="errorActivityWayland"
        autoDetect={true}
        language="en"
      />,
    );
    expect(markup).toContain('role="alert"');
    expect(markup).toContain('Wayland');
  });

  it('renders fallback unavailable notice when status is unavailable without a specific code', () => {
    const markup = renderToStaticMarkup(
      <ActivityIssueAlert
        status="unavailable"
        activityIssue={null}
        autoDetect={true}
        language="ja"
      />,
    );
    expect(markup).toContain('role="alert"');
    expect(markup).toContain('手動モード');
  });

  it('hides the alert completely when autoDetect is disabled (manual mode)', () => {
    expect(
      renderToStaticMarkup(
        <ActivityIssueAlert
          status="paused"
          activityIssue="errorActivityWayland"
          autoDetect={false}
          language="en"
        />,
      ),
    ).toBe('');
    expect(
      renderToStaticMarkup(
        <ActivityIssueAlert
          status="unavailable"
          activityIssue="errorActivityX11"
          autoDetect={false}
          language="en"
        />,
      ),
    ).toBe('');
  });
});
