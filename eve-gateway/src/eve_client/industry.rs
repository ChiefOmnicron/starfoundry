use crate::{Cache, Error, EveApiClient, IndustryJobEntry, IndustrySystem};

impl EveApiClient {
    /// Gets all industry jobs the corporation has running
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of industry jobs from the corp
    ///
    pub async fn industry_character_jobs(
        &self,
    ) -> Result<Vec<IndustryJobEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/characters/{}/industry/jobs",
            authenticated.character_id,
        );

        let response = self
            .fetch_page_auth::<IndustryJobEntry>(&path, Cache::Follow)
            .await
            .map_err(Into::into)?;

        Ok(response)
    }

    /// TODO: remove after the old collector is shut down
    /// Make sure the other function doens't follow cache rules
    pub async fn industry_character_jobs_copy(
        &self,
    ) -> Result<Vec<IndustryJobEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/characters/{}/industry/jobs",
            authenticated.character_id,
        );

        let response = self
            .fetch_page_auth::<IndustryJobEntry>(&path, Cache::Ignore)
            .await
            .map_err(Into::into)?;

        Ok(response)
    }

    /// Gets all industry jobs the corporation has running
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of industry jobs from the corp
    ///
    pub async fn industry_corporation_jobs(
        &self,
    ) -> Result<Vec<IndustryJobEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/corporations/{}/industry/jobs",
            authenticated.corporation_id
        );

        let mut response = self
            .fetch_page_auth::<IndustryJobEntry>(&path, Cache::Follow)
            .await
            .map_err(Into::into)?;

        for x in response.iter_mut() {
            x.corporation_id = Some(authenticated.corporation_id);
        }
        Ok(response)
    }

    /// TODO: remove after the old collector is shut down
    /// Make sure the other function doens't follow cache rules
    pub async fn industry_corporation_jobs_copy(
        &self,
    ) -> Result<Vec<IndustryJobEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/corporations/{}/industry/jobs",
            authenticated.corporation_id
        );

        let mut response = self
            .fetch_page_auth::<IndustryJobEntry>(&path, Cache::Ignore)
            .await
            .map_err(Into::into)?;

        for x in response.iter_mut() {
            x.corporation_id = Some(authenticated.corporation_id);
        }
        Ok(response)
    }

    /// Gets the industry index for all systems in eve
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [SystemId] is not a valid id
    /// 
    /// # Returns
    /// 
    /// All industry indicies for all different activities
    /// 
    pub async fn industry_index(
        &self,
    ) -> Result<Vec<IndustrySystem>, Error> {
        self
            .fetch::<Vec<IndustrySystem>>("latest/industry/systems", Cache::Follow)
            .await
            .map_err(Into::into)
    }
}
