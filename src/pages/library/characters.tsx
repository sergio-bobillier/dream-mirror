import Form from "next/form"
import { invoke } from '@tauri-apps/api/core';
import { Suspense } from "react";

import CharactersList from "../../components/characters-list";
import Layout from "../../components/layout";
import { Character } from "../../types/character"

export default async function Characters() {
  const characters = invoke("fetch_characters", {}) as Promise<Character[]>

  return (
    <Layout>
      <div className="page-wrapper">
        {/* BEGIN PAGE HEADER --> */}
        <div className="page-header d-print-none" aria-label="Page header">
          <div className="container-xl">
            <div className="row g-2 align-items-center">
              <div className="col">
                { /* Page pre-title */ }
                <div className="page-pretitle">Library</div>
                <h2 className="page-title">Characters</h2>
              </div>
              { /* Page title actions */ }
              <div className="col-sm-auto col-md-8 ms-auto d-print-none">
                <Form action="">
                  <div className="d-flex">
                    <input type="search" className="form-control d-inline-block w-9" placeholder="Search characters..." autoComplete="off" />
                  </div>
                </Form>
              </div>
            </div>
          </div>
        </div>
        {/* END PAGE HEADER */}
        {/* BEGIN PAGE BODY */}
        <div className="page-body">
          <div className="container-xl">
            <Suspense fallback={<div>Loading...</div>}>
              <CharactersList characters={characters} />
            </Suspense>
          </div>
        </div>
        { /* END PAGE BODY */ }
      </div>
    </Layout>
  )
}
