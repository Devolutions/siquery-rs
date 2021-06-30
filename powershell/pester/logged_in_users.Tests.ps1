Import-Module "$PSScriptRoot/../siquery"

Describe 'logged_in_users table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$logged_in_users = $allSchemas | Where-Object { $_.BaseName -eq 'logged_in_users' }
				$logged_in_users | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$logged_in_users = $allSchemas | Where-Object { $_.BaseName -eq 'logged_in_users' }
				$schema = Get-Content $logged_in_users.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$logged_in_users = Get-Siq 'logged_in_users' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'logged_in_users' }).FullName -Raw

				$json = $logged_in_users | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
