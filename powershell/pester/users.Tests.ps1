Import-Module "$PSScriptRoot/../siquery"

Describe 'users table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$users = $allSchemas | Where-Object { $_.BaseName -eq 'users' }
				$users | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$users = $allSchemas | Where-Object { $_.BaseName -eq 'users' }
				$schema = Get-Content $users.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$users = Get-Siq 'users' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'users' }).FullName -Raw

				$json = $users | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
