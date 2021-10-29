use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use tokio::{sync::mpsc, task::JoinHandle, task::unconstrained, time::Instant};
use futures::FutureExt;

pub enum Event<I> {
    Input(I),
    Tick,
}

#[allow(dead_code)]
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    input_handle: JoinHandle<()>,
    tick_handle: JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Events {
    pub async fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel(1);

        let input_handle = {
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();

                loop {
                    let timeout = config
                        .tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));

                    if event::poll(timeout).unwrap() {
                        if let Ok(CEvent::Key(key)) = event::read() {
                            if let Err(err) = tx.send(Event::Input(key)).await {
                                eprintln!("{}", err);
                                return;
                            }
                        }
                    }

                    if last_tick.elapsed() >= config.tick_rate {
                        if let Err(err) = tx.send(Event::Tick).await {
                            eprintln!("{}", err);
                            return;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };

        let tick_handle = {
            tokio::spawn(async move {
                loop {
                    if tx.send(Event::Tick).await.is_err() {
                        break;
                    }
                    tokio::time::sleep(config.tick_rate).await;
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub async fn next(&mut self) -> Option<Event<KeyEvent>> {
        unconstrained(self.rx.recv()).now_or_never().and_then(|f| f)
    }
}
