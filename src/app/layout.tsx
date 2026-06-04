'use client'

import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

import "./globals.css";

async function exitApp() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("exit_app", {});
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useEffect(() => {
    if (typeof document === 'undefined') return
    require('@tabler/core/dist/js/tabler')
  }, [])

  return (
    <html lang="en">
      <body className="layout-fluid">
        <div className="page">
          <div className="sticky-top">
            <header className="navbar-expand-md">
              <div className="navbar">
                <div className="container-xl">
                  <div className="row flex-column flex-md-row flex-fill align-items-center">
                    <div className="col">
                      {/* BEGIN NAVBAR MENU */}
                      <ul className="navbar-nav">
                        <li className="nav-item dropdown">
                          <a className="nav-link dropdown-toggle" href="#navbar-base" data-bs-toggle="dropdown" data-bs-auto-close="outside" role="button" aria-expanded="false">
                            <span className="nav-link-icon d-md-none d-lg-inline-block">
                              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="icon icon-1">
                                <path d="M12 3l8 4.5l0 9l-8 4.5l-8 -4.5l0 -9l8 -4.5"></path>
                                <path d="M12 12l8 -4.5"></path>
                                <path d="M12 12l0 9"></path>
                                <path d="M12 12l-8 -4.5"></path>
                                <path d="M16 5.25l-8 4.5"></path></svg>
                            </span>
                            <span className="nav-link-title"> Dream Mirror </span>
                          </a>
                          <div className="dropdown-menu">
                            <div className="dropdown-menu-columns">
                              <div className="dropdown-menu-column">
                                <a className="dropdown-item" onClick={() => { exitApp() }} href="#">
                                  Exit
                                </a>
                              </div>
                            </div>
                          </div>
                        </li>
                      </ul>
                      {/* END NAVBAR MENU */}
                    </div>
                    <div className="col col-md-auto">
                      <ul className="navbar-nav">
                        <li className="nav-item">
                          <a className="nav-link" href="#" data-bs-toggle="offcanvas" data-bs-target="#offcanvasSettings">
                            <span className="badge badge-sm bg-red text-red-fg">New</span>
                            <span className="nav-link-icon d-md-none d-lg-inline-block">
                              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="icon icon-1">
                                <path d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z"></path>
                                <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0"></path>
                              </svg>
                            </span>
                            <span className="nav-link-title"> Preferences </span>
                          </a>
                        </li>
                      </ul>
                    </div>
                  </div>
                </div>
              </div>
            </header>
          </div>
          <div className="page-wrapper" id="page-wrapper">
            {children}
          </div>
        </div>
      </body>
    </html>
  );
}
