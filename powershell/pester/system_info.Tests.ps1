Import-Module "$PSScriptRoot/../siquery"

Describe 'system_info table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$system_info = $allSchemas | Where-Object { $_.BaseName -eq 'system_info' }
				$system_info | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$system_info = $allSchemas | Where-Object { $_.BaseName -eq 'system_info' }
				$schema = Get-Content $system_info.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$system_info = Get-Siq 'system_info' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'system_info' }).FullName -Raw

				$json = $system_info | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
