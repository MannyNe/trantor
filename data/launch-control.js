class LaunchControl {
  /**
   * The server URL.
   * @private
   * @type {string}
   */
  serverUrl;

  /**
   * The last session state. Possible values are "start", "end", or null.
   * @private
   * @type {("start" | "end" | null)}
   */
  lastSessionState;

  /**
   * The tracking ID.
   * @private
   * @type {string}
   */
  trackingId;

  /**
   * Creates an instance of LaunchControl.
   * @param {string} serverUrl - The server URL.
   * @param {string} trackingId - The tracking ID.
   */
  constructor(serverUrl, trackingId) {
    this.serverUrl = serverUrl;
    this.lastSessionState = null;
    this.trackingId = trackingId;
  }

  /**
   * Starts a session.
   * @private
   */
  sessionStart() {
    const lastSessionIsNullOrEnd =
      this.lastSessionState === null || this.lastSessionState === "end";

    if (!lastSessionIsNullOrEnd) return;

    this.sendSessionStartBeacon();
    this.lastSessionState = "start";

    console.log("session start");
  }

  /**
   * Extracts the source name from the URL.
   * @private
   * @returns {Promise<string|null>} The source name or null.
   */
  async extractSourceName() {
    const url = new URL(window.location.href);
    if (!url.searchParams.has("src")) return null;

    const src = url.searchParams.get("src");
    window.history.replaceState({}, "", url.pathname);
    return src;
  }

  /**
   * Sends a session start beacon.
   * @private
   * @returns {Promise<void>}
   */
  async sendSessionStartBeacon() {
    const url = `${this.serverUrl}/session/start`;

    const headers = new Headers();
    headers.append("Content-Type", "application/json");
    headers.append("x-tracking-id", this.trackingId);
    const sourceName = await this.extractSourceName();

    if (sourceName) {
      headers.append("x-source-name", sourceName);
    }

    await fetch(url, {
      method: "POST",
      keepalive: true,
      credentials: "include",
      headers,
      body: JSON.stringify({
        timestamp: Date.now() / 1000, // seconds
        pathname: window.location.pathname,
        title: document.title,
      }),
    });
  }

  /**
   * Ends the current session.
   * @private
   */
  sessionEnd() {
    const lastSessionWasStart = this.lastSessionState === "start";

    if (!lastSessionWasStart) return;

    this.sendSessionEndBeacon();
    this.lastSessionState = "end";

    console.log("session end");
  }

  /**
   * Sends a session end beacon.
   * @private
   * @returns {Promise<void>}
   */
  async sendSessionEndBeacon() {
    await fetch(`${this.serverUrl}/session/end`, {
      method: "POST",
      keepalive: true,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        timestamp: Date.now() / 1000, // seconds
      }),
    });
  }

  /**
   * Initializes the Launch Control instance.
   * @public
   */
  initialize() {
    this.sessionStart();

    window.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "visible") {
        this.sessionStart();
      } else if (document.visibilityState === "hidden") {
        this.sessionEnd();
      }
    });

    window.addEventListener("blur", () => {
      this.sessionEnd();
    });

    window.addEventListener("focus", () => {
      this.sessionStart();
    });
  }

  /**
   * Sends a click event to the server.
   * @public
   * @param {string} buttonLabel - The label of the clicked button.
   * @returns {Promise<void>}
   */
  async sendClickEvent(buttonLabel) {
    await fetch(`${this.serverUrl}/session/event`, {
      method: "POST",
      keepalive: true,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        "x-tracking-id": this.trackingId,
      },
      body: JSON.stringify({
        type: "click",
        target: buttonLabel,
      }),
    });
  }
}

window.addEventListener("load", () => {
  const me = document.querySelector("script[data-id=trantor]");
  const serverUrl = new URL(me.src).origin;
  const trackingId = me.dataset.trackingId;

  window.launchControl = new LaunchControl(serverUrl, trackingId);
  window.launchControl.initialize();
});
